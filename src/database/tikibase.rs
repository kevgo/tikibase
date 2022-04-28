use super::{Directory, Document};
use crate::scan::{section_capitalization, section_level};
use crate::{Config, Issue};
use ahash::AHashMap;
use std::ffi::OsStr;
use std::path::PathBuf;

pub struct Tikibase {
    pub root: PathBuf,
    pub dir: Directory,
}

impl Tikibase {
    /// populates the gives issues vector with all issues found in this Tikibase

    pub fn check(&self) -> Vec<Issue> {
        let mut issues = vec![];
        let mut linked_resources = vec![];
        let mut title_variants = AHashMap::new();
        let mut level_variants = AHashMap::new();
        // round 1
        self.dir.check_1(
            &PathBuf::from(""),
            &mut issues,
            &mut linked_resources,
            &mut title_variants,
            &mut level_variants,
            &self.dir,
        );
        // analyze
        let title_outliers = section_capitalization::process(title_variants);
        let level_outliers = section_level::process(level_variants);
        // round 2
        self.dir.check_2(
            &PathBuf::from(""),
            &linked_resources,
            &mut issues,
            &title_outliers,
            &level_outliers,
        );
        issues.sort();
        issues
    }

    pub fn load(root: PathBuf) -> Result<Tikibase, Vec<Issue>> {
        let dir = Directory::load(&root, Config::default())?;
        Ok(Tikibase { root, dir })
    }

    pub fn get_doc<P: AsRef<OsStr>>(&self, relative_path: P) -> Option<&Document> {
        self.dir.get_doc(relative_path)
    }

    /// provides the document with the given relative filename as a mutable reference
    pub fn get_doc_mut<P: AsRef<OsStr>>(&mut self, path: P) -> Option<&mut Document> {
        self.dir.get_doc_mut(path)
    }
}

#[cfg(test)]
mod tests {
    use crate::{test, Issue, Location, Tikibase};

    #[test]
    fn missing_links() {
        let dir = test::tmp_dir();
        test::create_file("1.md", "# One\n\ntext\n", &dir);
        test::create_file("2.md", "# Two\n\n[one](1.md)\n", &dir);
        test::create_file("3.md", "# Three\n\n[one](1.md)\n", &dir);
        test::create_file("tikibase.json", r#"{ "bidiLinks": true }"#, &dir);
        let base = Tikibase::load(dir).unwrap();
        let have = base.check();
        let want = vec![
            Issue::DocumentWithoutLinks {
                location: Location {
                    file: "1.md".into(),
                    line: 0,
                    start: 0,
                    end: 0,
                },
            },
            Issue::MissingLink {
                location: Location {
                    file: "1.md".into(),
                    line: 2,
                    start: 0,
                    end: 0,
                },
                path: "2.md".into(),
                title: "Two".into(),
            },
            Issue::MissingLink {
                location: Location {
                    file: "1.md".into(),
                    line: 2,
                    start: 0,
                    end: 0,
                },
                path: "3.md".into(),
                title: "Three".into(),
            },
        ];
        pretty::assert_eq!(have, want);
    }

    #[test]
    fn obsolete_occurrences() {
        let dir = test::tmp_dir();
        test::create_file("1.md", "# One\n\ntext\n### occurrences\n\n- foo", &dir);
        test::create_file("tikibase.json", r#"{ "bidiLinks": true }"#, &dir);
        let base = Tikibase::load(dir).unwrap();
        let have = base.check();
        let want = vec![
            Issue::DocumentWithoutLinks {
                location: Location {
                    file: "1.md".into(),
                    line: 0,
                    start: 0,
                    end: 0,
                },
            },
            Issue::ObsoleteOccurrencesSection {
                location: Location {
                    file: "1.md".into(),
                    line: 3,
                    start: 4,
                    end: 15,
                },
            },
        ];
        pretty::assert_eq!(have, want);
    }
}
