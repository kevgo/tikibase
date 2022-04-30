use crate::check::{Issue, Location};
use crate::database::Document;
use ahash::AHashMap;

/// populates the given issues list with all duplicate sections in this document
pub(crate) fn scan(doc: &Document, issues: &mut Vec<Issue>) {
    // section title -> locations of sections with this title
    let mut sections_lines: AHashMap<&str, Vec<LocationWithinFile>> = AHashMap::new();
    for section in doc.sections() {
        sections_lines
            .entry(section.human_title())
            .or_insert_with(Vec::new)
            .push(LocationWithinFile {
                line: section.line_number,
                start: section.title_text_start as u32,
                end: section.title_text_end(),
            });
    }
    for (title, locations) in sections_lines.drain() {
        if locations.len() > 1 {
            for loc in locations {
                issues.push(Issue::DuplicateSection {
                    location: Location {
                        file: doc.relative_path.clone(),
                        line: loc.line,
                        start: loc.start,
                        end: loc.end,
                    },
                    title: title.into(),
                });
            }
        }
    }
}

struct LocationWithinFile {
    line: u32,
    start: u32,
    end: u32,
}

#[cfg(test)]
mod tests {
    use crate::check::{Issue, Location};
    use crate::database::Document;
    use indoc::indoc;

    #[test]
    fn has_duplicate_sections() {
        let content = indoc! {"
            # test document

            ### One
            content
            ### One
            content"};
        let doc = Document::from_str("test.md", content).unwrap();
        let mut have = vec![];
        super::scan(&doc, &mut have);
        let want = vec![
            Issue::DuplicateSection {
                location: Location {
                    file: "test.md".into(),
                    line: 2,
                    start: 4,
                    end: 7,
                },
                title: "One".into(),
            },
            Issue::DuplicateSection {
                location: Location {
                    file: "test.md".into(),
                    line: 4,
                    start: 4,
                    end: 7,
                },
                title: "One".into(),
            },
        ];
        pretty::assert_eq!(have, want);
    }

    #[test]
    fn no_duplicate_sections() {
        let content = indoc! {"
            # test document

            ### One
            content

            ### Two
            content"};
        let doc = Document::from_str("test.md", content).unwrap();
        let mut have = vec![];
        super::scan(&doc, &mut have);
        let want = vec![];
        pretty::assert_eq!(have, want);
    }
}
