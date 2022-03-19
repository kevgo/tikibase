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
    use crate::testhelpers::{create_file, empty_config, tmp_dir};

    mod get_doc {
        use super::super::Tikibase;
        use crate::testhelpers::{create_file, empty_config, tmp_dir};

        #[test]
        fn exists() {
            let dir = tmp_dir();
            create_file("one.md", "# test doc", &dir);
            let base = Tikibase::load(dir, &empty_config()).unwrap();
            let doc = base.get_doc("one.md").expect("document not found");
            assert_eq!(doc.title_section.title_line.text(), "# test doc");
        }

        #[test]
        fn missing() {
            let base = Tikibase::load(tmp_dir(), &empty_config()).unwrap();
            assert!(base.get_doc("zonk.md").is_none());
        }
    }

    mod get_doc_mut {
        use super::super::Tikibase;
        use crate::testhelpers::{create_file, empty_config, tmp_dir};

        #[test]
        fn exists() {
            let dir = tmp_dir();
            create_file("one.md", "# test doc", &dir);
            let mut base = Tikibase::load(dir, &empty_config()).unwrap();
            let doc = base.get_doc_mut("one.md").expect("document not found");
            assert_eq!(doc.title_section.title_line.text(), "# test doc");
        }

        #[test]
        fn missing() {
            let mut base = Tikibase::load(tmp_dir(), &empty_config()).unwrap();
            assert!(base.get_doc_mut("zonk.md").is_none());
        }
    }

    mod has_resource {
        use super::super::Tikibase;
        use crate::testhelpers::{create_file, empty_config, tmp_dir};

        #[test]
        fn empty() {
            let base = Tikibase::load(tmp_dir(), &empty_config()).unwrap();
            assert!(!base.has_resource("foo.png"));
        }

        #[test]
        fn matching_resource() {
            let dir = tmp_dir();
            create_file("foo.png", "content", &dir);
            let base = Tikibase::load(dir, &empty_config()).unwrap();
            assert!(base.has_resource("foo.png"));
        }
    }

    #[test]
    fn link_targets() {
        let dir = tmp_dir();
        let content = "\
# One

### Alpha
### Beta

content";
        create_file("one.md", content, &dir);
        create_file("two.md", content, &dir);
        let base = Tikibase::load(dir, &empty_config()).unwrap();
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
        assert_eq!(have, want);
    }

    #[test]
    fn load() {
        let dir = tmp_dir();
        let content = "\
# Title
title text
### Section 1
one
two
### Section 2
foo
";
        create_file("file.md", content, &dir);
        let base = Tikibase::load(dir, &empty_config()).unwrap();
        let doc_paths: Vec<String> = base
            .docs
            .iter()
            .map(|d| d.path.to_string_lossy().to_string())
            .collect();
        assert_eq!(doc_paths, vec!["file.md"]);
        let doc = &base.docs[0];
        // verify title of doc 0
        assert_eq!(doc.title_section.title_line.text(), "# Title");
        assert_eq!(doc.title_section.line_number, 0);
        let body: Vec<&str> = doc.title_section.body.iter().map(|l| l.text()).collect();
        assert_eq!(body, vec!["title text"]);
        // verify body of doc 0
        let content_sections: Vec<&str> = doc
            .content_sections
            .iter()
            .map(|s| s.title_line.text())
            .collect();
        assert_eq!(content_sections, vec!["### Section 1", "### Section 2"]);
        assert_eq!(doc.content_sections[0].line_number, 2);
        let sec0_lines: Vec<&str> = doc.content_sections[0]
            .body
            .iter()
            .map(|l| l.text())
            .collect();
        assert_eq!(sec0_lines, vec!["one", "two"]);
        assert_eq!(doc.content_sections[1].line_number, 5);
        let sec1_lines: Vec<&str> = doc.content_sections[1]
            .body
            .iter()
            .map(|l| l.text())
            .collect();
        assert_eq!(sec1_lines, vec!["foo"]);
    }

    #[test]
    fn load_hidden_file() {
        let dir = tmp_dir();
        create_file(".hidden", "content", &dir);
        let base = Tikibase::load(dir, &empty_config()).unwrap();
        assert_eq!(base.resources.len(), 0);
    }

    #[test]
    fn empty() {
        let base = Tikibase::load(tmp_dir(), &empty_config()).unwrap();
        assert_eq!(base.docs.len(), 0);
        assert_eq!(base.resources.len(), 0);
    }
}
