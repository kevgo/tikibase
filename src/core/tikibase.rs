use super::document::Document;
use super::resource::Resource;
use super::tikibase;
use rand::Rng;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct Tikibase {
    pub dir: PathBuf,
    pub docs: Vec<Document>,
    pub resources: Vec<Resource>,
}

impl Tikibase {
    /// creates a new document with the given content in this Tikibase
    pub fn create_doc(&mut self, filename: PathBuf, content: &str) {
        create_file(&self.dir.join(&filename), content);
        self.docs.push(Document::from_str(filename, content));
    }

    /// creates a new document with the given content in this Tikibase
    pub fn create_resource(&mut self, filename: PathBuf, content: &str) {
        create_file(&self.dir.join(&filename), content);
        self.resources.push(Resource { path: filename });
    }

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
    pub fn load_base(dir: PathBuf) -> Tikibase {
        let mut docs = Vec::new();
        let mut resources = Vec::new();
        for entry in WalkDir::new(&dir) {
            let entry = entry.unwrap();
            let filename = entry.file_name().to_str().unwrap();
            if filename == "." || filename == "tikibase.json" {
                continue;
            }
            let path = entry.into_path().strip_prefix(&dir).unwrap().to_owned();
            match DocType::from_ext(path.extension()) {
                DocType::Document => docs.push(tikibase::load_doc(path)),
                DocType::Resource => resources.push(Resource { path }),
            }
        }
        Tikibase {
            dir,
            docs,
            resources,
        }
    }

    /// creates a Tikibase instance for testing in the './tmp' directory
    pub fn tmpbase() -> Tikibase {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let rand: String = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(3)
            .map(char::from)
            .collect();
        let dir = std::path::PathBuf::from(format!("./tmp/{}-{}", timestamp, rand));
        match std::fs::create_dir_all(&dir) {
            Ok(_) => Tikibase::load_base(dir),
            Err(e) => panic!("{}", e),
        }
    }
}

enum DocType {
    Document,
    Resource,
}

impl DocType {
    fn from_ext(ext: Option<&std::ffi::OsStr>) -> DocType {
        match ext {
            None => DocType::Resource,
            Some(ext) => match ext.to_str() {
                Some("md") => DocType::Document,
                _ => DocType::Resource,
            },
        }
    }
}

fn create_file(filepath: &Path, content: &str) {
    let mut file = fs::File::create(&filepath).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

/// provides a Document instance containing the content of the file at the given path
fn load_doc(path: PathBuf) -> Document {
    let file = File::open(&path).unwrap();
    Document::from_lines(BufReader::new(file).lines().map(|l| l.unwrap()), path)
}

/// persists the current content of this document to disk
///
/// NOTE: this exists outside of Tikibase because of borrow  checker issues.
/// No point in borrowing the entire Tikibase object when we just need its root directory.
/// Also, this doesn't really belong into the Tikibase object nor the document object.
pub fn save_doc(doc: &Document, root: &Path) {
    let mut file = std::fs::File::create(root.join(&doc.path)).unwrap();
    file.write_all(doc.text().as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {

    use super::tikibase;
    use super::Tikibase;
    use std::path::PathBuf;

    mod get_doc {

        use super::super::Tikibase;
        use std::path::PathBuf;

        #[test]
        fn exists() {
            let mut base = Tikibase::tmpbase();
            base.create_doc(PathBuf::from("one.md"), "# test doc");
            let doc = base
                .get_doc(&PathBuf::from("one.md"))
                .expect("document not found");
            assert_eq!(doc.title_section.title_line.text, "# test doc");
        }

        #[test]
        fn missing() {
            let base = Tikibase::tmpbase();
            match base.get_doc(&PathBuf::from("zonk.md")) {
                None => return,
                Some(_) => panic!("should have found nothing"),
            }
        }
    }

    mod has_resource {

        use super::super::Tikibase;
        use std::path::PathBuf;

        #[test]
        fn empty() {
            let base = Tikibase::tmpbase();
            assert_eq!(base.has_resource(PathBuf::from("foo.png")), false);
        }

        #[test]
        fn matching_resource() {
            let mut base = Tikibase::tmpbase();
            base.create_resource(PathBuf::from("foo.png"), "content");
            assert_eq!(base.has_resource(PathBuf::from("foo.png")), true);
        }
    }

    #[test]
    fn link_targets() {
        let mut base = Tikibase::tmpbase();
        let content = "\
# One

### Alpha
### Beta

content";
        base.create_doc(PathBuf::from("one.md"), content);
        base.create_doc(PathBuf::from("two.md"), content);
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
        let content = "\
# Title
title text
### Section 1
one
two
### Section 2
foo
";
        let tmp_dir = tempfile::tempdir().unwrap();
        let file_path = tmp_dir.path().join("file.md");
        std::fs::write(&file_path, content).unwrap();
        let have = tikibase::load_doc(file_path);
        assert_eq!(have.title_section.title_line.text, "# Title");
        assert_eq!(have.title_section.line_number, 0);
        assert_eq!(have.title_section.body.len(), 1);
        assert_eq!(have.title_section.body[0].text, "title text");
        assert_eq!(have.title_section.body[0].section_offset, 1);
        assert_eq!(have.content_sections.len(), 2);
        assert_eq!(have.content_sections[0].title_line.text, "### Section 1");
        assert_eq!(have.content_sections[0].line_number, 2);
        assert_eq!(have.content_sections[0].body.len(), 2);
        assert_eq!(have.content_sections[0].body[0].text, "one");
        assert_eq!(have.content_sections[0].body[0].section_offset, 1);
        assert_eq!(have.content_sections[0].body[1].text, "two");
        assert_eq!(have.content_sections[0].body[1].section_offset, 2);
        assert_eq!(have.content_sections[1].title_line.text, "### Section 2");
        assert_eq!(have.content_sections[1].line_number, 5);
        assert_eq!(have.content_sections[1].body.len(), 1);
        assert_eq!(have.content_sections[1].body[0].text, "foo");
        assert_eq!(have.content_sections[1].body[0].section_offset, 1);
    }
}
