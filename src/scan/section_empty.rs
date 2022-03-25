use crate::{Issue, Location, Tikibase};

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
                    location: Location {
                        file: doc.path.clone(),
                        line: section.line_number,
                        start: 0,
                        end: section.title_line.text().len() as u32,
                    },
                    title: section.title().text.into(),
                });
            }
        }
    }
    issues
}

#[cfg(test)]
mod tests {
    use super::scan;
    use crate::{test, Config, Location};
    use crate::{Issue, Tikibase};
    use indoc::indoc;
    use std::path::PathBuf;

    #[test]
    fn empty_section() {
        let dir = test::tmp_dir();
        let content = indoc! {"
            # test document

            ### empty section
            ### next section

            content"};
        test::create_file("test.md", content, &dir);
        let base = Tikibase::load(dir, &Config::default()).unwrap();
        let have = scan(&base);
        let want = vec![Issue::EmptySection {
            location: Location {
                file: PathBuf::from("test.md"),
                line: 2,
                start: 0,
                end: 17,
            },
            title: "empty section".into(),
        }];
        pretty::assert_eq!(have, want);
    }

    #[test]
    fn blank_line() {
        let dir = test::tmp_dir();
        let content = indoc! {"
            # test document

            ### empty section

            ### next section

            content"};
        test::create_file("test.md", content, &dir);
        let base = Tikibase::load(dir, &Config::default()).unwrap();
        let have = scan(&base);
        let want = vec![Issue::EmptySection {
            location: Location {
                file: PathBuf::from("test.md"),
                line: 2,
                start: 0,
                end: 17,
            },
            title: "empty section".into(),
        }];
        pretty::assert_eq!(have, want);
    }

    #[test]
    fn content() {
        let dir = test::tmp_dir();
        let content = indoc! {"
            # test document

            ### section with content

            content"};
        test::create_file("test.md", content, &dir);
        let base = Tikibase::load(dir, &Config::default()).unwrap();
        let have = scan(&base);
        assert!(have.is_empty());
    }
}
