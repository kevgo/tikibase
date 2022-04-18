use super::{Directory, Document, DocumentsIterator};
use crate::{Config, Issue};
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub struct Tikibase {
    pub path: PathBuf,
    pub dir: Directory,
}

impl Tikibase {
    /// all existing resources with normalized name
    pub fn all_resources(&self) -> Vec<&OsString> {
        let mut result: Vec<&OsString> = vec![];
        self.dir
            .existing_resources(&mut result, OsString::from_str(""));
        result
    }

    pub fn documents(&self) -> DocumentsIterator {
        self.dir.documents()
    }

    /// provides the document with the given relative filename
    pub fn find_doc<P: AsRef<Path>>(&self, relative_path: P) -> Option<&Document> {
        let components = relative_path.as_ref().components();
        self.dir.find_doc(components.next().unwrap(), components)
    }

    /// provides the document with the given relative filename as a mutable reference
    pub fn find_doc_mut<P: AsRef<Path>>(&mut self, relative_path: P) -> Option<&mut Document> {
        let components = relative_path.as_ref().components();
        self.dir
            .find_doc_mut(components.next().unwrap(), components)
    }

    /// indicates whether the given link originating in the given Document points to something
    pub fn has_link_target(&self, doc: &Document, link: &str) -> LinkTargetResult {
        let iter = link.split('/');
        self.dir.has_link_target(iter.next().unwrap(), iter)
    }

    /// indicates whether this Tikibase contains a resource with the given path
    pub fn has_resource(&self, doc: &Document, link: &str) -> bool {
        let iter = link.split('/');
        self.dir.has_resource(iter.next().unwrap(), iter)
    }

    /// provides a Tikibase instance for the given directory
    pub fn load(path: PathBuf, config: &Config) -> Result<Tikibase, Vec<Issue>> {
        Ok(Tikibase {
            path,
            dir: Directory::load(&path)?,
        })
    }
}

pub enum LinkTargetResult {
    /// the given link target exists
    Exists,
    /// the given file doesn't exist
    NoFile(String),
    /// a directory doesn't exist
    NoDir(String),
    /// the given file exists but the given anchor in it doesn't exist
    NoAnchor,
    /// the given link target points to a resource with an anchor, which isn't supported
    ResourceWithAnchor,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;
    use indoc::indoc;

    #[test]
    fn empty() {
        let dir = test::tmp_dir();
        let base = Tikibase::load(dir, &Config::default()).unwrap();
        assert_eq!(base.documents().count(), 0);
        assert_eq!(base.resources().count(), 0);
    }

    mod find_doc {
        use crate::{test, Config, Tikibase};

        #[test]
        fn exists() {
            let dir = test::tmp_dir();
            test::create_file("one.md", "# test doc", &dir);
            let base = Tikibase::load(dir, &Config::default()).unwrap();
            let doc = base.find_doc("one.md").expect("document not found");
            assert_eq!(doc.title_section.title_line.text, "# test doc");
        }

        #[test]
        fn missing() {
            let dir = test::tmp_dir();
            let base = Tikibase::load(dir, &Config::default()).unwrap();
            assert!(base.find_doc("zonk.md").is_none());
        }
    }

    mod get_doc_mut {
        use crate::{test, Config, Tikibase};

        #[test]
        fn exists() {
            let dir = test::tmp_dir();
            test::create_file("one.md", "# test doc", &dir);
            let mut base = Tikibase::load(dir, &Config::default()).unwrap();
            let doc = base.find_doc_mut("one.md").expect("document not found");
            assert_eq!(doc.title_section.title_line.text, "# test doc");
        }

        #[test]
        fn missing() {
            let dir = test::tmp_dir();
            let mut base = Tikibase::load(dir, &Config::default()).unwrap();
            assert!(base.find_doc_mut("zonk.md").is_none());
        }
    }

    mod has_resource {
        use crate::{test, Config, Tikibase};

        #[test]
        fn empty() {
            let dir = test::tmp_dir();
            let base = Tikibase::load(dir, &Config::default()).unwrap();
            assert!(!base.has_resource("foo.png"));
        }

        #[test]
        fn matching_resource() {
            let dir = test::tmp_dir();
            test::create_file("foo.png", "content", &dir);
            let base = Tikibase::load(dir, &Config::default()).unwrap();
            assert!(base.has_resource("foo.png"));
        }

        // TODO: test resource in subdir
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
        let base = Tikibase::load(dir, &Config::default()).unwrap();
        // make sure we can load existing documents
        let _doc = &base.find_doc("file.md").unwrap();
    }

    #[test]
    fn load_hidden_file() {
        let dir = test::tmp_dir();
        test::create_file(".hidden", "content", &dir);
        let base = Tikibase::load(dir, &Config::default()).unwrap();
        assert_eq!(base.resources().count(), 0);
    }
}
