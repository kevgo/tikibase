use crate::{Issue, Position, Tikibase};

/// finds all duplicate sections in the given Tikibase
pub(crate) fn scan(base: &Tikibase) -> Vec<Issue> {
    let mut issues = Vec::new();
    for doc in &base.docs {
        let mut known_sections = Vec::new();
        for section in &doc.content_sections {
            let section_type = section.section_type();
            if known_sections.contains(&section_type) {
                issues.push(Issue::DuplicateSection {
                    pos: Position {
                        file: doc.path.clone(),
                        line: section.line_number,
                    },
                    section_type: section_type.into(),
                });
            } else {
                known_sections.push(section_type);
            }
        }
    }
    issues
}

#[cfg(test)]
mod tests {
    use super::scan;
    use crate::{test, Config, Issue, Position, Tikibase};
    use std::path::PathBuf;

    #[test]
    fn duplicate_sections() {
        let dir = test::tmp_dir();
        let content = "\
# test document

### One
content
### One
content";
        test::create_file("test.md", content, &dir);
        let base = Tikibase::load(dir, &Config::default()).unwrap();
        let have = scan(&base);
        pretty::assert_eq!(
            have,
            vec![Issue::DuplicateSection {
                pos: Position {
                    file: PathBuf::from("test.md"),
                    line: 6
                },
                section_type: "One".into(),
            }]
        )
    }
}
