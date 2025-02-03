use crate::check::{Issue, Location};
use crate::database::Document;
use gxhash::{HashMap, HashMapExt};

/// populates the given issues list with all duplicate sections in this document
pub fn scan(doc: &Document, issues: &mut Vec<Issue>) {
  // section title -> locations of sections with this title
  let mut sections_lines: HashMap<&str, Vec<LocationWithinFile>> = HashMap::new();
  for section in doc.sections() {
    sections_lines
      .entry(section.human_title())
      .or_default()
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
  use big_s::S;
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
          file: S("test.md"),
          line: 2,
          start: 4,
          end: 7,
        },
        title: S("One"),
      },
      Issue::DuplicateSection {
        location: Location {
          file: S("test.md"),
          line: 4,
          start: 4,
          end: 7,
        },
        title: S("One"),
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
