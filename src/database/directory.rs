use super::{paths, Document};
use crate::check::Issue;
use crate::config::LoadResult;
use crate::{config, Config};
use ahash::AHashMap;
use fs_err as fs;
use merge::Merge;

pub struct Directory {
    pub relative_path: String,
    pub config: Config,
    pub dirs: AHashMap<String, Directory>,
    pub docs: AHashMap<String, Document>,
    pub resources: AHashMap<String, ()>,
}

impl Directory {
    /// provides the document with the given relative filename
    pub fn get_doc<OS: AsRef<str>>(&self, relative_path: OS) -> Option<&Document> {
        self.docs.get(relative_path.as_ref())
    }

    /// provides the document with the given relative filename as a mutable reference
    pub fn get_doc_mut<OS: AsRef<str>>(&mut self, relative_path: OS) -> Option<&mut Document> {
        self.docs.get_mut(relative_path.as_ref())
    }

    /// indicates whether this Tikibase contains a directory with the given name
    pub fn has_dir<P: AsRef<str>>(&self, path: P) -> bool {
        self.dirs.contains_key(path.as_ref())
    }

    /// indicates whether this Tikibase contains a resource with the given path
    pub fn has_resource<P: AsRef<str>>(&self, path: P) -> bool {
        self.resources.contains_key(path.as_ref())
    }

    /// provides a Directory instance for the given directory
    pub fn load(
        root: &str,
        relative_path: String,
        mut parent_config: Config,
    ) -> Result<Directory, Vec<Issue>> {
        let abs_path = paths::join(root, &relative_path);
        let config = match config::load(&abs_path) {
            LoadResult::Loaded(config) => {
                parent_config.merge(config);
                parent_config
            }
            LoadResult::NotFound => parent_config,
            LoadResult::Error(issue) => return Err(vec![issue]),
        };
        let mut docs = AHashMap::new();
        let mut dirs = AHashMap::new();
        let mut resources = AHashMap::new();
        let mut errors = Vec::new();
        for entry in fs::read_dir(abs_path).unwrap() {
            let entry = entry.unwrap();
            let entry_path = entry.path();
            let entry_name = entry.file_name().to_string_lossy().to_string();
            match EntryType::from_direntry(&entry, &config) {
                EntryType::Document => match Document::load(&entry_path, entry_name.clone()) {
                    Ok(doc) => {
                        docs.insert(entry_name, doc);
                    }
                    Err(err) => errors.push(err),
                },
                EntryType::Resource => {
                    resources.insert(entry_name, ());
                }
                EntryType::Configuration | EntryType::Ignored => continue,
                EntryType::Directory => {
                    dirs.insert(
                        entry_name.clone(),
                        Directory::load(root, entry_name, config.clone())?,
                    );
                }
            }
        }
        if errors.is_empty() {
            Ok(Directory {
                relative_path,
                config,
                dirs,
                docs,
                resources,
            })
        } else {
            Err(errors)
        }
    }
}

/// filesystem entry types that Tikibase distinguishes
#[derive(Debug, PartialEq)]
pub enum EntryType {
    /// subdirectory of the current directory
    Directory,
    /// Markdown document
    Document,
    /// linkable resource
    Resource,
    /// Tikibase configuration file
    Configuration,
    /// ignored file
    Ignored,
}

impl EntryType {
    fn from_direntry(entry: &fs::DirEntry, config: &Config) -> EntryType {
        let entry_type = entry.file_type().unwrap();
        let entry_filename_os = entry.file_name();
        let entry_filename = entry_filename_os.to_string_lossy();
        if entry_type.is_file() {
            if entry_filename == "tikibase.json" {
                return EntryType::Configuration;
            }
            if entry_filename.starts_with('.') {
                return EntryType::Ignored;
            }
            if config.ignore(&entry_filename) {
                return EntryType::Ignored;
            }
            if has_extension(&entry_filename, "md") {
                return EntryType::Document {};
            }
            return EntryType::Resource;
        }
        if entry_type.is_dir() {
            return EntryType::Directory;
        }
        EntryType::Ignored
    }

    pub fn from_str(path: &str) -> EntryType {
        if path == "tikibase.json" {
            return EntryType::Configuration;
        }
        if path.starts_with('.') {
            return EntryType::Ignored;
        }
        if has_extension(path, "md") {
            return EntryType::Document;
        }
        if path.ends_with('/') {
            return EntryType::Directory;
        }
        EntryType::Resource
    }
}

/// case-insensitive comparison of file extensions
fn has_extension(path: &str, given_ext: &str) -> bool {
    let path_ext = path.rsplit('.').next().unwrap();
    path_ext.eq_ignore_ascii_case(given_ext)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;
    use indoc::indoc;

    #[test]
    fn empty() {
        let dir = test::tmp_dir();
        let dir = Directory::load(&dir, "".into(), Config::default()).unwrap();
        assert_eq!(dir.docs.len(), 0);
        assert_eq!(dir.resources.len(), 0);
    }

    mod entry_type {
        use crate::database::EntryType;

        #[test]
        fn from_str() {
            let tests = vec![
                ("foo.md", EntryType::Document),
                ("sub/foo.md", EntryType::Document),
                ("foo.png", EntryType::Resource),
                ("foo.pdf", EntryType::Resource),
                (".testconfig.json", EntryType::Ignored),
                ("dir/", EntryType::Directory),
            ];
            for (give, want) in tests {
                let have = EntryType::from_str(give);
                assert_eq!(have, want);
            }
        }
    }

    mod get_doc {
        use crate::database::Directory;
        use crate::{test, Config};

        #[test]
        fn exists() {
            let dir = test::tmp_dir();
            test::create_file("one.md", "# test doc", &dir);
            let dir = Directory::load(&dir, "".into(), Config::default()).unwrap();
            let doc = dir.get_doc("one.md").unwrap();
            assert_eq!(doc.title_section.title_line.text, "# test doc");
        }

        #[test]
        fn missing() {
            let dir = test::tmp_dir();
            let dir = Directory::load(&dir, "".into(), Config::default()).unwrap();
            assert!(dir.get_doc("zonk.md").is_none());
        }
    }

    mod get_doc_mut {
        use crate::database::Directory;
        use crate::{test, Config};

        #[test]
        fn exists() {
            let dir = test::tmp_dir();
            test::create_file("one.md", "# test doc", &dir);
            let mut dir = Directory::load(&dir, "".into(), Config::default()).unwrap();
            let doc = dir.get_doc_mut("one.md").unwrap();
            assert_eq!(doc.title_section.title_line.text, "# test doc");
        }

        #[test]
        fn missing() {
            let dir = test::tmp_dir();
            let mut dir = Directory::load(&dir, "".into(), Config::default()).unwrap();
            assert!(dir.get_doc_mut("zonk.md").is_none());
        }
    }

    #[test]
    fn has_extension() {
        let tests = vec![
            (("foo.md", "md"), true),
            (("FOO.MD", "md"), true),
            (("foo.md", "MD"), true),
            (("foo.md", "png"), false),
        ];
        for (give, want) in tests {
            let have = super::has_extension(give.0, give.1);
            assert_eq!(have, want);
        }
    }

    mod has_resource {
        use crate::database::Directory;
        use crate::{test, Config};

        #[test]
        fn empty() {
            let dir = test::tmp_dir();
            let dir = Directory::load(&dir, "".into(), Config::default()).unwrap();
            assert!(!dir.has_resource("foo.png"));
        }

        #[test]
        fn matching_resource() {
            let dir = test::tmp_dir();
            test::create_file("foo.png", "content", &dir);
            let dir = Directory::load(&dir, "".into(), Config::default()).unwrap();
            assert!(dir.has_resource("foo.png"));
        }
    }

    #[test]
    fn load() {
        let dir = test::tmp_dir();
        let content = indoc! {"
            # Title
            title text
            ### Section 1
            one
            two
            ### Section 2
            foo
            "};
        test::create_file("file.md", content, &dir);
        let dir = Directory::load(&dir, "".into(), Config::default()).unwrap();
        // make sure we can load existing documents
        let _doc = &dir.get_doc("file.md").unwrap();
    }

    #[test]
    fn load_hidden_file() {
        let dir = test::tmp_dir();
        test::create_file(".hidden", "content", &dir);
        let dir = Directory::load(&dir, "".into(), Config::default()).unwrap();
        assert_eq!(dir.resources.len(), 0);
    }
}
