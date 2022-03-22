use crate::{Issue, Position, Tikibase};
use ahash::AHashMap;

/// finds all duplicate sections in the given Tikibase
pub(crate) fn scan(base: &Tikibase) -> Vec<Issue> {
    let mut issues = Vec::new();
    for doc in &base.docs {
        // section title -> [lines with this section]
        let mut sections_lines: AHashMap<&str, Vec<u32>> = AHashMap::new();
        for section in doc.sections() {
            sections_lines
                .entry(section.section_type())
                .or_insert_with(Vec::new)
                .push(section.line_number)
        }
        for (section_type, section_lines) in sections_lines.drain() {
            if section_lines.len() > 1 {
                for line in section_lines {
                    issues.push(Issue::DuplicateSection {
                        pos: Position {
                            file: doc.path.clone(),
                            line,
                        },
                        section_type: section_type.into(),
                    });
                }
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
            vec![
                Issue::DuplicateSection {
                    pos: Position {
                        file: PathBuf::from("test.md"),
                        line: 2
                    },
                    section_type: "One".into(),
                },
                Issue::DuplicateSection {
                    pos: Position {
                        file: PathBuf::from("test.md"),
                        line: 4
                    },
                    section_type: "One".into(),
                },
            ]
        )
    }
}
