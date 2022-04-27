use crate::{Document, Issue, Location};
use ahash::AHashMap;
use std::path::Path;

/// populates the given issues list with all duplicate sections in this document
pub(crate) fn scan(doc: &Document, path: &Path, issues: &mut Vec<Issue>) {
    // section title -> [lines with this section]
    let mut sections_lines: AHashMap<&str, Vec<(u32, u32, u32)>> = AHashMap::new();
    for section in doc.sections() {
        sections_lines
            .entry(section.human_title())
            .or_insert_with(Vec::new)
            .push((
                section.line_number,
                section.title_text_start as u32,
                section.title_text_end(),
            ));
    }
    for (title, lines) in sections_lines.drain() {
        if lines.len() > 1 {
            for (line, start, end) in lines {
                issues.push(Issue::DuplicateSection {
                    location: Location {
                        file: path.into(),
                        line,
                        start,
                        end,
                    },
                    title: title.into(),
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Document, Issue, Location};
    use indoc::indoc;
    use std::path::PathBuf;

    #[test]
    fn with_duplicate_sections() {
        let content = indoc! {"
            # test document

            ### One
            content
            ### One
            content"};
        let doc = Document::from_str("test.md", content).unwrap();
        let mut have = vec![];
        super::scan(&doc, &PathBuf::from("test.md"), &mut have);
        let want = vec![
            Issue::DuplicateSection {
                location: Location {
                    file: PathBuf::from("test.md"),
                    line: 2,
                    start: 4,
                    end: 7,
                },
                title: "One".into(),
            },
            Issue::DuplicateSection {
                location: Location {
                    file: PathBuf::from("test.md"),
                    line: 4,
                    start: 4,
                    end: 7,
                },
                title: "One".into(),
            },
        ];
        pretty::assert_eq!(have, want);
    }
}
