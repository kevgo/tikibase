use super::Document;
use crate::config::LoadResult;
use crate::{config, Config, Issue};
use ahash::AHashMap;
use merge::Merge;
use std::ffi::{OsStr, OsString};
use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;

pub struct Directory {
    pub config: Config,
    pub dirs: AHashMap<OsString, Directory>,
    pub docs: AHashMap<OsString, Document>,
    pub resources: AHashMap<OsString, ()>,
}

impl Directory {
    /// provides the document with the given relative filename
    pub fn get_doc<OS: AsRef<OsStr>>(&self, relative_path: OS) -> Option<&Document> {
        self.docs.get(relative_path.as_ref())
    }

    /// provides the document with the given relative filename as a mutable reference
    pub fn get_doc_mut<OS: AsRef<OsStr>>(&mut self, relative_path: OS) -> Option<&mut Document> {
        self.docs.get_mut(relative_path.as_ref())
    }

    /// indicates whether this Tikibase contains a resource with the given path
    pub fn has_resource<P: AsRef<OsStr>>(&self, path: P) -> bool {
        self.resources.contains_key(path.as_ref())
    }

    /// provides all valid link targets in this Tikibase
    // TODO: populate a given accumulator
    pub fn link_targets(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for (_path, doc) in &self.docs {
            let filename = doc.relative_path.to_string_lossy().to_string();
            for section in doc.sections() {
                result.push(format!("{}{}", &filename, section.anchor()));
            }
            result.push(filename);
        }
        result.sort();
        result
    }

    /// provides a Tikibase instance for the given directory
    pub fn load(dir: &Path, mut parent_config: Config) -> Result<Directory, Vec<Issue>> {
        let config = match config::load(dir) {
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
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let entry_path = entry.path();
            let entry_name = entry.file_name();
            match EntryType::from_direntry(&entry, &config) {
                EntryType::Document => {
                    let file = File::open(&entry_path).unwrap();
                    match Document::from_reader(BufReader::new(file), entry_name.clone()) {
                        Ok(doc) => {
                            docs.insert(entry_name, doc);
                        }
                        Err(err) => errors.push(err),
                    }
                }
                EntryType::Resource => {
                    resources.insert(entry_name, ());
                }
                EntryType::Configuration | EntryType::Ignored => continue,
                EntryType::Directory => {
                    dirs.insert(entry_name, Directory::load(&entry_path, config.clone())?);
                }
            }
        }
        if errors.is_empty() {
            Ok(Directory {
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

/// types of files that Tikibase is aware of
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
        let entry_os_filename = entry.file_name();
        if entry_type.is_file() {
            if entry_os_filename == "tikibase.json" {
                return EntryType::Configuration;
            }
            let entry_filestr = entry_os_filename.to_string_lossy();
            if entry_filestr.starts_with('.') {
                return EntryType::Ignored;
            }
            if config.ignore(&entry_filestr) {
                return EntryType::Ignored;
            }
            if has_extension(&entry_filestr, "md") {
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
        let dir = Directory::load(&dir, Config::default()).unwrap();
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
            let dir = Directory::load(&dir, Config::default()).unwrap();
            let doc = dir.get_doc("one.md").expect("document not found");
            assert_eq!(doc.title_section.title_line.text, "# test doc");
        }

        #[test]
        fn missing() {
            let dir = test::tmp_dir();
            let dir = Directory::load(&dir, Config::default()).unwrap();
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
            let mut dir = Directory::load(&dir, Config::default()).unwrap();
            let doc = dir.get_doc_mut("one.md").expect("document not found");
            assert_eq!(doc.title_section.title_line.text, "# test doc");
        }

        #[test]
        fn missing() {
            let dir = test::tmp_dir();
            let mut dir = Directory::load(&dir, Config::default()).unwrap();
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
            let dir = Directory::load(&dir, Config::default()).unwrap();
            assert!(!dir.has_resource("foo.png"));
        }

        #[test]
        fn matching_resource() {
            let dir = test::tmp_dir();
            test::create_file("foo.png", "content", &dir);
            let dir = Directory::load(&dir, Config::default()).unwrap();
            assert!(dir.has_resource("foo.png"));
        }
    }

    #[test]
    fn link_targets() {
        let dir = test::tmp_dir();
        let content = indoc! {"
            # One

            ### Alpha
            ### Beta

            content"};
        test::create_file("one.md", content, &dir);
        test::create_file("two.md", content, &dir);
        let dir = Directory::load(&dir, Config::default()).unwrap();
        let have = dir.link_targets();
        let want = vec![
            "one.md",
            "one.md#alpha",
            "one.md#beta",
            "one.md#one",
            "two.md",
            "two.md#alpha",
            "two.md#beta",
            "two.md#one",
        ];
        pretty::assert_eq!(have, want);
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
        let dir = Directory::load(&dir, Config::default()).unwrap();
        // make sure we can load existing documents
        let _doc = &dir.get_doc("file.md").unwrap();
    }

    #[test]
    fn load_hidden_file() {
        let dir = test::tmp_dir();
        test::create_file(".hidden", "content", &dir);
        let dir = Directory::load(&dir, Config::default()).unwrap();
        assert_eq!(dir.resources.len(), 0);
    }
}
