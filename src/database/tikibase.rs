use super::Document;
use super::Resource;
use crate::config;
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
    pub fn has_resource<P: AsRef<Path>>(&self, filename: P) -> bool {
        self.resources
            .iter()
            .any(|resource| resource.path == filename.as_ref())
    }

    /// provides all valid link targets in this Tikibase
    pub fn link_targets(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for doc in &self.docs {
            let filename = doc.path.to_string_lossy().to_string();
            result.push(format!("{}{}", &filename, doc.title_section.anchor()));
            for section in &doc.content_sections {
                result.push(format!("{}{}", &filename, section.anchor()));
            }
            result.push(filename);
        }
        result.sort();
        result
    }

    /// Provides a Tikibase instance for the given directory.
    pub fn load(dir: PathBuf, config: &config::Data) -> (Tikibase, Vec<String>) {
        let mut docs = Vec::new();
        let mut resources = Vec::new();
        let mut errors = Vec::new();
        for entry in WalkDir::new(&dir) {
            let entry = entry.unwrap();
            if entry.path() == dir {
                continue;
            }
            let filename = entry.file_name().to_string_lossy();
            if filename == "tikibase.json" || filename.starts_with('.') {
                continue;
            }
            match &config.ignore {
                Some(ignore) => {
                    if ignore.iter().any(|i| i == &filename) {
                        continue;
                    }
                }
                None => {}
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
        (
            Tikibase {
                dir,
                docs,
                resources,
            },
            errors,
        )
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
            let (base, errs) = Tikibase::load(dir, &empty_config());
            assert_eq!(errs.len(), 0);
            let doc = base.get_doc("one.md").expect("document not found");
            assert_eq!(doc.title_section.title_line.text(), "# test doc");
        }

        #[test]
        fn missing() {
            let dir = tmp_dir();
            let (base, errs) = Tikibase::load(dir, &empty_config());
            assert_eq!(errs.len(), 0);
            match base.get_doc("zonk.md") {
                None => {}
                Some(_) => panic!("should have found nothing"),
            }
        }
    }

    mod get_doc_mut {

        use super::super::Tikibase;
        use crate::testhelpers::{create_file, empty_config, tmp_dir};

        #[test]
        fn exists() {
            let dir = tmp_dir();
            create_file("one.md", "# test doc", &dir);
            let (mut base, errs) = Tikibase::load(dir, &empty_config());
            assert_eq!(errs.len(), 0);
            let doc = base.get_doc_mut("one.md").expect("document not found");
            assert_eq!(doc.title_section.title_line.text(), "# test doc");
        }

        #[test]
        fn missing() {
            let dir = tmp_dir();
            let (mut base, errs) = Tikibase::load(dir, &empty_config());
            assert_eq!(errs.len(), 0);
            match base.get_doc_mut("zonk.md") {
                None => {}
                Some(_) => panic!("should have found nothing"),
            }
        }
    }

    mod has_resource {

        use super::super::Tikibase;
        use crate::testhelpers::{create_file, empty_config, tmp_dir};

        #[test]
        fn empty() {
            let dir = tmp_dir();
            let (base, errs) = Tikibase::load(dir, &empty_config());
            assert_eq!(errs.len(), 0);
            assert!(!base.has_resource("foo.png"));
        }

        #[test]
        fn matching_resource() {
            let dir = tmp_dir();
            create_file("foo.png", "content", &dir);
            let (base, errs) = Tikibase::load(dir, &empty_config());
            assert_eq!(errs.len(), 0);
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
        let (base, errs) = Tikibase::load(dir, &empty_config());
        assert_eq!(errs.len(), 0);
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
        let (base, errs) = Tikibase::load(dir, &empty_config());
        assert_eq!(errs.len(), 0);
        assert_eq!(base.docs.len(), 1);
        let doc = &base.docs[0];
        assert_eq!(doc.path.to_string_lossy(), "file.md");
        assert_eq!(doc.title_section.title_line.text(), "# Title");
        assert_eq!(doc.title_section.line_number, 0);
        assert_eq!(doc.title_section.body.len(), 1);
        assert_eq!(doc.title_section.body[0].text(), "title text");
        assert_eq!(doc.content_sections.len(), 2);
        assert_eq!(doc.content_sections[0].title_line.text(), "### Section 1");
        assert_eq!(doc.content_sections[0].line_number, 2);
        assert_eq!(doc.content_sections[0].body.len(), 2);
        assert_eq!(doc.content_sections[0].body[0].text(), "one");
        assert_eq!(doc.content_sections[0].body[1].text(), "two");
        assert_eq!(doc.content_sections[1].title_line.text(), "### Section 2");
        assert_eq!(doc.content_sections[1].line_number, 5);
        assert_eq!(doc.content_sections[1].body.len(), 1);
        assert_eq!(doc.content_sections[1].body[0].text(), "foo");
    }

    #[test]
    fn load_hidden_file() {
        let dir = tmp_dir();
        create_file(".hidden", "content", &dir);
        let (base, errs) = Tikibase::load(dir, &empty_config());
        assert_eq!(errs.len(), 0);
        assert_eq!(base.resources.len(), 0);
    }

    #[test]
    fn empty() {
        let dir = tmp_dir();
        let (base, errs) = Tikibase::load(dir, &empty_config());
        assert_eq!(errs.len(), 0);
        assert_eq!(base.docs.len(), 0);
        assert_eq!(base.resources.len(), 0);
    }
}
