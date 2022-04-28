use crate::scan::{section_capitalization, section_level};
use crate::{Outcome, Tikibase};
use ahash::AHashMap;
use std::path::PathBuf;

pub fn check(base: &Tikibase) -> Outcome {
    let mut issues = vec![];
    let mut linked_resources = vec![];
    let mut title_variants = AHashMap::new();
    let mut level_variants = AHashMap::new();
    // round 1
    base.dir.check_1(
        &PathBuf::from(""),
        &mut issues,
        &mut linked_resources,
        &mut title_variants,
        &mut level_variants,
        &base.dir,
    );
    // analyze
    let title_outliers = section_capitalization::process(title_variants);
    let level_outliers = section_level::process(level_variants);
    // round 2
    base.dir.check_2(
        &linked_resources,
        &mut issues,
        &title_outliers,
        &level_outliers,
    );
    issues.sort();
    Outcome {
        issues,
        fixes: vec![],
    }
}

#[cfg(test)]
mod tests {
    use crate::{test, Issue, Location, Outcome, Tikibase};

    #[test]
    fn missing_links() {
        let dir = test::tmp_dir();
        test::create_file("1.md", "# One\n\ntext\n", &dir);
        test::create_file("2.md", "# Two\n\n[one](1.md)\n", &dir);
        test::create_file("3.md", "# Three\n\n[one](1.md)\n", &dir);
        test::create_file("tikibase.json", r#"{ "bidiLinks": true }"#, &dir);
        let base = Tikibase::load(dir).unwrap();
        let have = super::check(&base);
        let want = Outcome {
            issues: vec![
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
            ],
            fixes: vec![],
        };
        pretty::assert_eq!(have, want);
    }

    #[test]
    fn obsolete_occurrences() {
        let dir = test::tmp_dir();
        test::create_file("1.md", "# One\n\ntext\n### occurrences\n\n- foo", &dir);
        test::create_file("tikibase.json", r#"{ "bidiLinks": true }"#, &dir);
        let base = Tikibase::load(dir).unwrap();
        let have = super::check(&base);
        let want = Outcome {
            issues: vec![
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
            ],
            fixes: vec![],
        };
        pretty::assert_eq!(have, want);
    }
}
