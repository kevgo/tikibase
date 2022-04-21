use super::{Document, Resource};
use crate::config::LoadResult;
use crate::{config, Config, Issue};
use std::fs::{self, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};

pub struct Directory {
    pub config: Config,
    // TODO: make AHashMap<OsString, ()>
    pub dirs: Vec<Directory>,
    // TODO: make AHashMap<OsString, ()>
    pub docs: Vec<Document>,
    // TODO: make AHashMap<OsString, ()>
    pub resources: Vec<Resource>,
}

impl Directory {
    /// provides the document with the given relative filename
    pub fn get_doc<P: AsRef<Path>>(&self, relative_path: P) -> Option<&Document> {
        let relative_path = relative_path.as_ref();
        self.docs
            .iter()
            .find(|doc| doc.relative_path == relative_path)
    }

    /// provides the document with the given relative filename as a mutable reference
    pub fn get_doc_mut<P: AsRef<Path>>(&mut self, path: P) -> Option<&mut Document> {
        let path = path.as_ref();
        self.docs.iter_mut().find(|doc| doc.relative_path == path)
    }

    /// indicates whether this Tikibase contains a resource with the given path
    pub fn has_resource<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();
        self.resources.iter().any(|resource| resource.path == path)
    }

    /// provides all valid link targets in this Tikibase
    pub fn link_targets(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for doc in &self.docs {
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
    pub fn load(dir: &Path, parent_config: Config) -> Result<Directory, Vec<Issue>> {
        let config = match config::load(dir) {
            LoadResult::Loaded(config) => config, // TODO: merge with parent_config
            LoadResult::NotFound => {
                let config: Config = parent_config;
                config
            }
            LoadResult::Error(issue) => return Err(vec![issue]),
        };
        let mut docs = Vec::new();
        let mut dirs = Vec::new();
        let mut resources = Vec::new();
        let mut errors = Vec::new();
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let entry_path = entry.path();
            let entry_name = entry.file_name();
            match EntryType::from_direntry(&entry, &config) {
                EntryType::Document => {
                    let file = File::open(&entry_path).unwrap();
                    match Document::from_reader(BufReader::new(file), entry_name) {
                        Ok(doc) => docs.push(doc),
                        Err(err) => errors.push(err),
                    }
                }
                EntryType::Resource => {
                    resources.push(Resource {
                        path: PathBuf::from(entry_name),
                    });
                }
                EntryType::Configuration | EntryType::Ignored => continue,
                EntryType::Directory => dirs.push(Directory::load(&entry_path, config.clone())?), // TODO: try to borrow config here
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
