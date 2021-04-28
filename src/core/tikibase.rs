use super::document::Document;
use super::resource::Resource;
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
    pub fn get_doc(&self, filename: &Path) -> Option<&Document> {
        self.docs.iter().find(|doc| doc.path == filename)
    }

    /// indicates whether this Tikibase contains a resource with the given path
    pub fn has_resource(&self, filename: PathBuf) -> bool {
        self.resources
            .iter()
            .any(|resource| resource.path == filename)
    }

    /// provides all valid link targets in this Tikibase
    pub fn link_targets(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for doc in &self.docs {
            let filename = doc.path.to_string_lossy();
            result.push(format!("{}{}", &filename, doc.title_section.anchor()));
            for section in &doc.content_sections {
                result.push(format!("{}{}", &filename, section.anchor()));
            }
            result.push(filename.to_string());
        }
        result.sort();
        result
    }

    /// Provides a Tikibase instance for the given directory.
    pub fn load(dir: PathBuf) -> Tikibase {
        let mut docs = Vec::new();
        let mut resources = Vec::new();
        for entry in WalkDir::new(&dir) {
            let entry = entry.unwrap();
            if entry.path() == dir {
                continue;
            }
            let filename = entry.file_name().to_str().unwrap();
            if filename == "tikibase.json" || filename.starts_with(".") {
                continue;
            }
            let path = entry.path();
            let filepath = path.strip_prefix(&dir).unwrap().to_owned();
            match FileType::from_ext(path.extension()) {
                FileType::Document => {
                    let file = File::open(&path).unwrap();
                    docs.push(Document::from_lines(
                        BufReader::new(file).lines().map(|l| l.unwrap()),
                        filepath,
                    ));
                }
                FileType::Resource => resources.push(Resource { path: filepath }),
            }
        }
        Tikibase {
            dir,
            docs,
            resources,
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

        use super::super::*;
        use crate::testhelpers;
        use std::path::PathBuf;

        #[test]
        fn exists() {
            let dir = testhelpers::tmp_dir();
            testhelpers::create_file("one.md", "# test doc", &dir);
            let base = Tikibase::load(dir);
            let doc = base
                .get_doc(&PathBuf::from("one.md"))
                .expect("document not found");
            assert_eq!(doc.title_section.title_line.text, "# test doc");
        }

        #[test]
        fn missing() {
            let dir = testhelpers::tmp_dir();
            let base = Tikibase::load(dir);
            match base.get_doc(&PathBuf::from("zonk.md")) {
                None => return,
                Some(_) => panic!("should have found nothing"),
            }
        }
    }

    mod has_resource {

        use super::super::*;
        use crate::testhelpers;
        use std::path::PathBuf;

        #[test]
        fn empty() {
            let dir = testhelpers::tmp_dir();
            let base = Tikibase::load(dir);
            assert_eq!(base.has_resource(PathBuf::from("foo.png")), false);
        }

        #[test]
        fn matching_resource() {
            let dir = testhelpers::tmp_dir();
            testhelpers::create_file("foo.png", "content", &dir);
            let base = Tikibase::load(dir);
            assert_eq!(base.has_resource(PathBuf::from("foo.png")), true);
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
        let base = Tikibase::load(dir);
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
        let base = Tikibase::load(dir);
        assert_eq!(base.docs.len(), 1);
        let doc = &base.docs[0];
        assert_eq!(doc.path.to_string_lossy(), "file.md");
        assert_eq!(doc.title_section.title_line.text, "# Title");
        assert_eq!(doc.title_section.line_number, 0);
        assert_eq!(doc.title_section.body.len(), 1);
        assert_eq!(doc.title_section.body[0].text, "title text");
        assert_eq!(doc.title_section.body[0].section_offset, 1);
        assert_eq!(doc.content_sections.len(), 2);
        assert_eq!(doc.content_sections[0].title_line.text, "### Section 1");
        assert_eq!(doc.content_sections[0].line_number, 2);
        assert_eq!(doc.content_sections[0].body.len(), 2);
        assert_eq!(doc.content_sections[0].body[0].text, "one");
        assert_eq!(doc.content_sections[0].body[0].section_offset, 1);
        assert_eq!(doc.content_sections[0].body[1].text, "two");
        assert_eq!(doc.content_sections[0].body[1].section_offset, 2);
        assert_eq!(doc.content_sections[1].title_line.text, "### Section 2");
        assert_eq!(doc.content_sections[1].line_number, 5);
        assert_eq!(doc.content_sections[1].body.len(), 1);
        assert_eq!(doc.content_sections[1].body[0].text, "foo");
        assert_eq!(doc.content_sections[1].body[0].section_offset, 1);
    }

    #[test]
    fn load_hidden_file() {
        let dir = testhelpers::tmp_dir();
        testhelpers::create_file(".prettierrc", "semi: false", &dir);
        let base = Tikibase::load(dir);
        assert_eq!(base.resources.len(), 0);
    }

    #[test]
    fn empty() {
        let dir = testhelpers::tmp_dir();
        let base = Tikibase::load(dir);
        assert_eq!(base.docs.len(), 0);
        assert_eq!(base.resources.len(), 0);
    }
}
