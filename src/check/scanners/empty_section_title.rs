use crate::check::{Issue, Location};
use crate::database::Section;
use std::path::Path;

/// populates the given issues list if this section has an empty title
pub fn scan(section: &Section, path: &Path, issues: &mut Vec<Issue>) {
    if section.human_title().is_empty() {
        issues.push(Issue::SectionWithoutHeader {
            location: Location {
                file: path.into(),
                line: section.line_number,
                start: 0,
                end: section.title_text_end(),
            },
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::check::{Issue, Location};
    use crate::database::Document;
    use indoc::indoc;
    use std::path::PathBuf;

    #[test]
    fn empty_title() {
        let content = indoc! {"
            # test document

            ###
            content
            ###
            content"};
        let doc = Document::from_str("test.md", content).unwrap();
        let mut have = vec![];
        for section in doc.sections() {
            super::scan(section, &PathBuf::from("test.md"), &mut have);
        }
        let want = vec![
            Issue::SectionWithoutHeader {
                location: Location {
                    file: PathBuf::from("test.md"),
                    line: 2,
                    start: 0,
                    end: 3,
                },
            },
            Issue::SectionWithoutHeader {
                location: Location {
                    file: PathBuf::from("test.md"),
                    line: 4,
                    start: 0,
                    end: 3,
                },
            },
        ];
        pretty::assert_eq!(have, want);
    }
}
