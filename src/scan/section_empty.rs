use crate::{Issue, Position, Tikibase};

/// finds all empty sections in the given Tikibase,
/// fixes them if fix is enabled,
/// returns the unfixed issues
pub(crate) fn scan(base: &Tikibase) -> Vec<Issue> {
    let mut issues = Vec::<Issue>::new();
    for doc in &base.docs {
        for section in &doc.content_sections {
            let has_content = section.body.iter().any(|line| !line.text().is_empty());
            if !has_content {
                issues.push(Issue::EmptySection {
                    pos: Position {
                        file: doc.path.clone(),
                        line: section.line_number,
                    },
                    section_type: section.section_type().into(),
                });
            }
        }
    }
    issues
}

#[cfg(test)]
mod tests {
    use super::scan;
    use crate::{test, Config, Position};
    use crate::{Issue, Tikibase};
    use std::path::PathBuf;

    #[test]
    fn empty_section() {
        let dir = test::tmp_dir();
        let content = "\
# test document

### empty section
### next section

content";
        test::create_file("test.md", content, &dir);
        let base = Tikibase::load(dir, &Config::default()).unwrap();
        let have = scan(&base);
        let want = vec![Issue::EmptySection {
            pos: Position {
                file: PathBuf::from("test.md"),
                line: 2,
            },
            section_type: "empty section".into(),
        }];
        pretty::assert_eq!(have, want);
    }

    #[test]
    fn blank_line() {
        let dir = test::tmp_dir();
        let content = "\
# test document

### empty section

### next section

content";
        test::create_file("test.md", content, &dir);
        let base = Tikibase::load(dir, &Config::default()).unwrap();
        let have = scan(&base);
        let want = vec![Issue::EmptySection {
            pos: Position {
                file: PathBuf::from("test.md"),
                line: 2,
            },
            section_type: "empty section".into(),
        }];
        pretty::assert_eq!(have, want)
    }

    #[test]
    fn content() {
        let dir = test::tmp_dir();
        let content = "\
# test document

### section with content

content";
        test::create_file("test.md", content, &dir);
        let base = Tikibase::load(dir, &Config::default()).unwrap();
        let have = scan(&base);
        assert!(have.is_empty());
    }
}
