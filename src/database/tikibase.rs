use super::{Directory, Document, DocumentsIterator, ResourceIterator};
use crate::{Config, Issue};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

pub struct Tikibase {
    pub path: PathBuf,
    pub dir: Directory,
}

impl Tikibase {
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

    pub fn has_link_target<P: AsRef<Path>>(&self, relative_path: P) -> LinkTargetResult {
        let components = relative_path.as_ref().components();
        self.dir
            .has_link_target(components.next().unwrap(), components)
    }

    /// indicates whether this Tikibase contains a resource with the given path
    pub fn has_resource<P: AsRef<Path>>(&self, relative_path: P) -> bool {
        let components = relative_path.as_ref().components();
        self.dir
            .has_resource(components.next().unwrap(), components)
    }

    /// provides a Tikibase instance for the given directory
    pub fn load(path: PathBuf, config: &Config) -> Result<Tikibase, Vec<Issue>> {
        Ok(Tikibase {
            path,
            dir: Directory::load(&path)?,
        })
    }

    pub fn resources(&self) -> ResourceIterator {
        self.dir.resources()
    }
}

pub enum LinkTargetResult {
    /// the given link target exists
    Exists,
    /// the given file doesn't exist
    NoFile(OsString),
    /// a directory doesn't exist
    NoDir(OsString),
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

    mod get_doc {
        use crate::{test, Config, Tikibase};

        #[test]
        fn exists() {
            let dir = test::tmp_dir();
            test::create_file("one.md", "# test doc", &dir);
            let base = Tikibase::load(dir, &Config::default()).unwrap();
            let doc = base.get_doc("one.md").expect("document not found");
            assert_eq!(doc.title_section.title_line.text, "# test doc");
        }

        #[test]
        fn missing() {
            let dir = test::tmp_dir();
            let base = Tikibase::load(dir, &Config::default()).unwrap();
            assert!(base.get_doc("zonk.md").is_none());
        }
    }

    mod get_doc_mut {
        use crate::{test, Config, Tikibase};

        #[test]
        fn exists() {
            let dir = test::tmp_dir();
            test::create_file("one.md", "# test doc", &dir);
            let mut base = Tikibase::load(dir, &Config::default()).unwrap();
            let doc = base.get_doc_mut("one.md").expect("document not found");
            assert_eq!(doc.title_section.title_line.text, "# test doc");
        }

        #[test]
        fn missing() {
            let dir = test::tmp_dir();
            let mut base = Tikibase::load(dir, &Config::default()).unwrap();
            assert!(base.get_doc_mut("zonk.md").is_none());
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
        let base = Tikibase::load(dir, &Config::default()).unwrap();
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
        let _doc = &base.get_doc("file.md").unwrap();
    }

    #[test]
    fn load_hidden_file() {
        let dir = test::tmp_dir();
        test::create_file(".hidden", "content", &dir);
        let base = Tikibase::load(dir, &Config::default()).unwrap();
        assert_eq!(base.resources.len(), 0);
    }
}
