use super::{Document, Resource};
use crate::{Config, Issue, Location};
use ignore::overrides::OverrideBuilder;
use ignore::WalkBuilder;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

pub struct Tikibase {
    pub dir: PathBuf,
    pub docs: Vec<Document>,
    pub resources: Vec<Resource>,
}

impl Tikibase {
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
    pub fn load(dir: PathBuf, config: &Config) -> Result<Tikibase, Vec<Issue>> {
        let mut docs = Vec::new();
        let mut resources = Vec::new();
        let mut errors = Vec::new();
        let mut override_builder = OverrideBuilder::new(&dir);
        if let Some(globs) = &config.globs {
            for glob in globs {
                if let Err(err) = override_builder.add(glob) {
                    return Err(vec![Issue::InvalidGlob {
                        glob: glob.into(),
                        location: Location {
                            file: PathBuf::from("tikibase.json"),
                            line: 0,
                            start: 0,
                            end: 0,
                        },
                        message: err.to_string(),
                    }]);
                }
            }
        }
        let over_ride = match override_builder.build() {
            Ok(o) => o,
            Err(err) => panic!("Cannot build glob overrides: {}", err),
        };
        let walker = WalkBuilder::new(&dir)
            .overrides(over_ride)
            .sort_by_file_path(Ord::cmp)
            .build();
        for entry in walker {
            let entry = entry.unwrap();
            if entry.path() == dir {
                continue;
            }
            let path = entry.path();
            let rel_path = path.strip_prefix(&dir).unwrap();
            match FileType::from(path) {
                FileType::Document => {
                    let file = File::open(&path).unwrap();
                    match Document::from_reader(BufReader::new(file), rel_path) {
                        Ok(doc) => docs.push(doc),
                        Err(err) => errors.push(err),
                    }
                }
                FileType::Resource => resources.push(Resource {
                    path: rel_path.into(),
                }),
                FileType::Configuration | FileType::Ignored => continue,
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

/// types of files that Tikibase is aware of
#[derive(Debug, PartialEq)]
pub enum FileType {
    /// Markdown document
    Document,
    /// linkable resource
    Resource,
    /// Tikibase configuration file
    Configuration,
    /// ignored file
    Ignored,
}

impl From<&String> for FileType {
    fn from(path: &String) -> Self {
        let p: &str = path.as_ref();
        FileType::from(p)
    }
}

impl From<&str> for FileType {
    fn from(path: &str) -> Self {
        if path == "tikibase.json" {
            return FileType::Configuration;
        }
        if path.starts_with('.') {
            return FileType::Ignored;
        }
        if has_extension(path, "md") {
            return FileType::Document;
        }
        FileType::Resource
    }
}

impl From<&Path> for FileType {
    fn from(path: &Path) -> FileType {
        FileType::from(path.file_name().unwrap().to_string_lossy().as_ref())
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
        let base = Tikibase::load(dir, &Config::default()).unwrap();
        assert_eq!(base.docs.len(), 0);
        assert_eq!(base.resources.len(), 0);
    }

    #[test]
    fn file_type() {
        let tests = vec![
            ("foo.md", FileType::Document),
            ("sub/foo.md", FileType::Document),
            ("foo.png", FileType::Resource),
            ("foo.pdf", FileType::Resource),
            (".testconfig.json", FileType::Ignored),
        ];
        for (give, want) in tests {
            let have = FileType::from(give);
            assert_eq!(have, want);
        }
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
