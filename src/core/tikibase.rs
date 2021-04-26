use super::document::Document;
use super::persistence;
use super::resource::Resource;
use std::path::{Path, PathBuf};

pub struct Tikibase {
    pub dir: PathBuf,
    pub docs: Vec<Document>,
    pub resources: Vec<Resource>,
}

impl Tikibase {
    /// creates a new document with the given content in this Tikibase
    pub fn create_doc(&mut self, filename: &Path, content: &str) {
        let filepath = self.dir.join(filename);
        persistence::save_file(&filepath, content);
        self.docs.push(Document::from_str(filepath, content));
    }

    /// creates a new document with the given content in this Tikibase
    pub fn create_resource(&mut self, filename: PathBuf, content: &str) {
        persistence::save_file(&self.dir.join(&filename), content);
        self.resources.push(Resource { path: filename });
    }

    /// provides the current content of the document with the given name
    pub fn doc_content(&self, filename: &Path) -> String {
        match self.docs.iter().find(|doc| doc.path == filename) {
            None => panic!("document not found: {:?}", &filename),
            Some(doc) => doc.text(),
        }
    }

    /// indicates whether this Tikibase contains a resource with the given path
    pub fn has_resource(&self, filename: &Path) -> bool {
        let filepath = self.dir.join(filename);
        self.resources
            .iter()
            .any(|resource| resource.path == filepath)
    }

    /// provides all valid link targets in this Tikibase
    pub fn link_targets(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for doc in &self.docs {
            let filename = doc.relative_path(&self.dir);
            result.push(format!("{}{}", &filename, doc.title_section.anchor()));
            for section in &doc.content_sections {
                result.push(format!("{}{}", &filename, section.anchor()));
            }
            result.push(filename);
        }
        result.sort();
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::core::persistence;
    use std::path::PathBuf;

    mod doc_content {

        use crate::core::persistence;
        use std::path::PathBuf;

        #[test]
        fn doc_content() {
            let mut base = persistence::tmpbase();
            let content = "# Test\ncontent";
            base.create_doc(&PathBuf::from("1.md"), &content);
            let have = base.doc_content(&PathBuf::from("1.md"));
            assert_eq!(have, content)
        }
    }

    mod has_resource {

        use crate::core::persistence;
        use std::path::PathBuf;

        #[test]
        fn empty() {
            let base = persistence::tmpbase();
            assert_eq!(base.has_resource(&PathBuf::from("foo.png")), false);
        }

        #[test]
        fn matching_resource() {
            let mut base = persistence::tmpbase();
            base.create_resource(&PathBuf::from("foo.png"), "content");
            assert_eq!(base.has_resource(&PathBuf::from("foo.png")), true);
        }
    }

    #[test]
    fn link_targets() {
        let mut base = persistence::tmpbase();
        let content = "\
# One

### Alpha
### Beta

content";
        base.create_doc(&PathBuf::from("one.md"), content);
        base.create_doc(&PathBuf::from("two.md"), content);
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
}
