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
    pub fn create_doc(&mut self, filename: PathBuf, content: &str) {
        persistence::save_file(&self.dir.join(&filename), content);
        self.docs.push(Document::from_str(filename, content));
    }

    /// creates a new document with the given content in this Tikibase
    pub fn create_resource(&mut self, filename: PathBuf, content: &str) {
        persistence::save_file(&self.dir.join(&filename), content);
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
}

#[cfg(test)]
mod tests {

    use crate::core::persistence;
    use std::path::PathBuf;

    mod get_doc {

        use crate::core::persistence;
        use std::path::PathBuf;

        #[test]
        fn exists() {
            let mut base = persistence::tmpbase();
            base.create_doc(PathBuf::from("one.md"), "# test doc");
            let doc = base
                .get_doc(&PathBuf::from("one.md"))
                .expect("document not found");
            assert_eq!(doc.title_section.title_line.text, "# test doc");
        }

        #[test]
        fn missing() {
            let base = persistence::tmpbase();
            match base.get_doc(&PathBuf::from("zonk.md")) {
                None => return,
                Some(_) => panic!("should have found nothing"),
            }
        }
    }

    mod has_resource {

        use crate::core::persistence;
        use std::path::PathBuf;

        #[test]
        fn empty() {
            let base = persistence::tmpbase();
            assert_eq!(base.has_resource(PathBuf::from("foo.png")), false);
        }

        #[test]
        fn matching_resource() {
            let mut base = persistence::tmpbase();
            base.create_resource(PathBuf::from("foo.png"), "content");
            assert_eq!(base.has_resource(PathBuf::from("foo.png")), true);
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
}
