use super::{Document, Resource};
use crate::{config, Issue};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct Tikibase {
    pub dir: PathBuf,
    pub docs: Vec<Document>,
    pub resources: Vec<Resource>,
}

impl Tikibase {
    /// provides the document with the given relative filename
    pub fn get_doc<P: AsRef<Path>>(&self, path: P) -> Option<&Document> {
        let path = path.as_ref();
        self.docs.iter().find(|doc| doc.path == path)
    }

    /// provides the document with the given relative filename as a mutable reference
    pub fn get_doc_mut<P: AsRef<Path>>(&mut self, path: P) -> Option<&mut Document> {
        let path = path.as_ref();
        self.docs.iter_mut().find(|doc| doc.path == path)
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
            let filename = doc.path.to_string_lossy().to_string();
            for section in doc.sections() {
                result.push(format!("{}{}", &filename, section.anchor()));
            }
            result.push(filename);
        }
        result.sort();
        result
    }

    /// provides a Tikibase instance for the given directory
    pub fn load(dir: PathBuf, config: &config::Data) -> Result<Tikibase, Vec<Issue>> {
        let mut docs = Vec::new();
        let mut resources = Vec::new();
        let mut errors = Vec::new();
        for entry in WalkDir::new(&dir) {
            let entry = entry.unwrap();
            if entry.path() == dir {
                continue;
            }
            let filename = entry.file_name().to_string_lossy();
            if filename.starts_with('.') || filename == "tikibase.json" {
                continue;
            }
            if let Some(ignore) = &config.ignore {
                // TODO: merge this line with the previous one once https://github.com/rust-lang/rust/issues/53667 ships
                if ignore.iter().any(|i| i == &filename) {
                    continue;
                }
            }
            let path = entry.path();
            let filepath = path.strip_prefix(&dir).unwrap();
            match FileType::from_ext(path.extension()) {
                FileType::Document => {
                    let file = File::open(&path).unwrap();
                    let lines = BufReader::new(file).lines().map(Result::unwrap);
                    match Document::from_lines(lines, filepath) {
                        Ok(doc) => docs.push(doc),
                        Err(err) => errors.push(err),
                    }
                }
                FileType::Resource => resources.push(Resource {
                    path: filepath.into(),
                }),
            }
        }
        if errors.is_empty() {
            Ok(Tikibase {
                dir,
                docs,
                resources,
            })
        } else {
            Err(errors)
        }
    }
}

enum FileType {
    Document,
    Resource,
}

impl FileType {
    fn from_ext(ext: Option<&std::ffi::OsStr>) -> FileType {
        match ext {
            None => FileType::Resource,
            Some(ext) => match ext.to_str() {
                Some("md") => FileType::Document,
                _ => FileType::Resource,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testhelpers;

    mod get_doc {
        use super::super::Tikibase;
        use crate::testhelpers;

        #[test]
        fn exists() {
            let dir = testhelpers::tmp_dir();
            testhelpers::create_file("one.md", "# test doc", &dir);
            let base = Tikibase::load(dir, &testhelpers::empty_config()).unwrap();
            let doc = base.get_doc("one.md").expect("document not found");
            assert_eq!(doc.title_section.title_line.text(), "# test doc");
        }

        #[test]
        fn missing() {
            let dir = testhelpers::tmp_dir();
            let base = Tikibase::load(dir, &testhelpers::empty_config()).unwrap();
            assert!(base.get_doc("zonk.md").is_none());
        }
    }

    mod get_doc_mut {
        use super::super::Tikibase;
        use crate::testhelpers;

        #[test]
        fn exists() {
            let dir = testhelpers::tmp_dir();
            testhelpers::create_file("one.md", "# test doc", &dir);
            let mut base = Tikibase::load(dir, &testhelpers::empty_config()).unwrap();
            let doc = base.get_doc_mut("one.md").expect("document not found");
            assert_eq!(doc.title_section.title_line.text(), "# test doc");
        }

        #[test]
        fn missing() {
            let mut base =
                Tikibase::load(testhelpers::tmp_dir(), &testhelpers::empty_config()).unwrap();
            assert!(base.get_doc_mut("zonk.md").is_none());
        }
    }

    mod has_resource {
        use super::super::Tikibase;
        use crate::testhelpers;

        #[test]
        fn empty() {
            let base =
                Tikibase::load(testhelpers::tmp_dir(), &testhelpers::empty_config()).unwrap();
            assert!(!base.has_resource("foo.png"));
        }

        #[test]
        fn matching_resource() {
            let dir = testhelpers::tmp_dir();
            testhelpers::create_file("foo.png", "content", &dir);
            let base = Tikibase::load(dir, &testhelpers::empty_config()).unwrap();
            assert!(base.has_resource("foo.png"));
        }
    }

    #[test]
    fn link_targets() {
        let dir = testhelpers::tmp_dir();
        let content = "\
# One

### Alpha
### Beta

content";
        testhelpers::create_file("one.md", content, &dir);
        testhelpers::create_file("two.md", content, &dir);
        let base = Tikibase::load(dir, &testhelpers::empty_config()).unwrap();
        let have = base.link_targets();
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
        let dir = testhelpers::tmp_dir();
        let content = "\
# Title
title text
### Section 1
one
two
### Section 2
foo
";
        testhelpers::create_file("file.md", content, &dir);
        let base = Tikibase::load(dir, &testhelpers::empty_config()).unwrap();
        // make sure we can load existing documents
        let _doc = &base.get_doc("file.md").unwrap();
    }

    #[test]
    fn load_hidden_file() {
        let dir = testhelpers::tmp_dir();
        testhelpers::create_file(".hidden", "content", &dir);
        let base = Tikibase::load(dir, &testhelpers::empty_config()).unwrap();
        assert_eq!(base.resources.len(), 0);
    }

    #[test]
    fn empty() {
        let base = Tikibase::load(testhelpers::tmp_dir(), &testhelpers::empty_config()).unwrap();
        assert_eq!(base.docs.len(), 0);
        assert_eq!(base.resources.len(), 0);
    }
}
