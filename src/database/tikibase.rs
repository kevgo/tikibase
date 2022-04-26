use super::{Directory, Document};
use crate::{Config, Issue};
use std::ffi::OsStr;
use std::path::PathBuf;

pub struct Tikibase {
    pub root: PathBuf,
    pub dir: Directory,
}

impl Tikibase {
    /// populates the gives issues vector with all issues found in this Tikibase
    pub fn check(&self, issues: &mut Vec<Issue>, linked_resources: &mut Vec<PathBuf>) {
        self.dir
            .check(&PathBuf::from(""), issues, linked_resources, &self.dir);
        self.check_round_2(linked_resources, issues);
    }

    /// populates the given `unlinked_resources` list with all resources in this Tikibase that aren't linked to
    pub fn check_round_2(&self, linked_resources: &[PathBuf], issues: &mut Vec<Issue>) {
        self.dir
            .check_round_2(&PathBuf::from(""), linked_resources, issues);
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
        let mut issues = vec![];
        let mut linked_resources = vec![];
        base.check(&mut issues, &mut linked_resources);
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
        issues.sort();
        pretty::assert_eq!(issues, want);
    }

    #[test]
    fn obsolete_occurrences() {
        let dir = test::tmp_dir();
        test::create_file("1.md", "# One\n\ntext\n### occurrences\n\n- foo", &dir);
        test::create_file("tikibase.json", r#"{ "bidiLinks": true }"#, &dir);
        let base = Tikibase::load(dir).unwrap();
        let mut issues = vec![];
        let mut linked_resources = vec![];
        base.check(&mut issues, &mut linked_resources);
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
        pretty::assert_eq!(issues, want);
    }
}
