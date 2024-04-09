use crate::check::{Issue, Location};
use crate::database::Section;

/// populates the given issues list if this section has no content
pub fn scan(section: &Section, path: &str, issues: &mut Vec<Issue>) {
  if section.is_empty() {
    issues.push(Issue::EmptySection {
      location: Location {
        file: path.into(),
        line: section.line_number,
        start: 0,
        end: section.title_line.text.len() as u32,
      },
      title: section.human_title().into(),
    });
  }
}

#[cfg(test)]
mod tests {
  use crate::check::{Issue, Location};
  use crate::database::Document;
  use big_s::S;
  use indoc::indoc;

  #[test]
  fn empty_section() {
    let content = indoc! {"
            # test document

            ### empty section
            ### next section

            content"};
    let doc = Document::from_str("test.md", content).unwrap();
    let mut have = vec![];
    for section in doc.content_sections {
      super::scan(&section, "test.md", &mut have);
    }
    let want = vec![Issue::EmptySection {
      location: Location {
        file: S("test.md"),
        line: 2,
        start: 0,
        end: 17,
      },
      title: S("empty section"),
    }];
    pretty::assert_eq!(have, want);
  }

  #[test]
  fn blank_line() {
    let content = indoc! {"
            # test document

            ### empty section

            ### next section

            content"};
    let doc = Document::from_str("test.md", content).unwrap();
    let mut have = vec![];
    for section in doc.content_sections {
      super::scan(&section, "test.md", &mut have);
    }
    let want = vec![Issue::EmptySection {
      location: Location {
        file: S("test.md"),
        line: 2,
        start: 0,
        end: 17,
      },
      title: S("empty section"),
    }];
    pretty::assert_eq!(have, want);
  }

  #[test]
  fn content() {
    let content = indoc! {"
            # test document

            ### section with content

            content"};
    let doc = Document::from_str("test.md", content).unwrap();
    let mut have = vec![];
    for section in doc.content_sections {
      super::scan(&section, "test.md", &mut have);
    }
    assert!(have.is_empty());
  }
}
