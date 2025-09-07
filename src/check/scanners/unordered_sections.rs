use crate::Config;
use crate::check::{Issue, Location};
use crate::database::Document;

/// populates the given issues list with all sections in this document that don't match the configured order
pub fn scan(doc: &Document, config: &Config, issues: &mut Vec<Issue>) {
  let Some(schema_titles) = &config.sections else {
    return;
  };
  if doc.content_sections.len() < 2 {
    // document has 0 or 1 sections --> order always matches
    return;
  }
  let mut sections_iter = doc.content_sections.iter();
  let mut section_option = sections_iter.next();
  let mut schema_iter = schema_titles.iter();
  let mut schema_option = schema_iter.next();
  loop {
    let Some(doc_section) = section_option else {
      return; // we reached the end of the actual list --> actual matches schema
    };
    let Some(schema_title) = schema_option else {
      // end of schema reached but there are still unchecked sections in the document --> those are out of order
      issues.push(Issue::UnorderedSections {
        location: Location {
          file: doc.relative_path.clone(),
          line: doc_section.line_number,
          start: 0,
          end: doc_section.title_line.text.len() as u32,
        },
      });
      section_option = sections_iter.next();
      continue;
    };
    let section_title = &doc_section.title_line.text;
    if section_title == schema_title {
      // elements match --> advance both pointers
      section_option = sections_iter.next();
      schema_option = schema_iter.next();
      continue;
    }
    // HACK: see https://github.com/rust-lang/rust/issues/42671
    if !schema_titles.iter().any(|st| st == section_title) {
      // unknown element in actual --> ignore here (there is a separate check for this)
      section_option = sections_iter.next();
      continue;
    }
    // elements don't match --> advance the schema
    // (because schema might contain elements that are not in actual)
    schema_option = schema_iter.next();
  }
}

#[cfg(test)]
mod tests {
  use crate::Config;
  use crate::check::{Issue, Location};
  use crate::database::Document;
  use big_s::S;
  use indoc::indoc;

  #[test]
  fn mismatching() {
    let content = indoc! {"
            # Test
            ### one
            text
            ### three
            text
            ### two
            text"};
    let doc = Document::from_str("test.md", content).unwrap();
    let config = Config {
      sections: Some(vec![S("### one"), S("### two"), S("### three")]),
      ..Config::default()
    };
    let mut issues = vec![];
    super::scan(&doc, &config, &mut issues);
    let want = vec![Issue::UnorderedSections {
      location: Location {
        file: S("test.md"),
        line: 5,
        start: 0,
        end: 7,
      },
    }];
    assert_eq!(issues, want);
  }

  #[test]
  fn perfect_match() {
    let content = indoc! {"
            # Test
            ### one
            text
            ### two
            text
            ### three
            text"};
    let doc = Document::from_str("test.md", content).unwrap();
    let config = Config {
      sections: Some(vec![S("one"), S("two"), S("three")]),
      ..Config::default()
    };
    let mut issues = vec![];
    super::scan(&doc, &config, &mut issues);
    let want = vec![];
    assert_eq!(issues, want);
  }

  #[test]
  fn match_but_missing() {
    let content = indoc! {"
            # Test
            ### one
            text
            ### two
            text
            ### three
            text"};
    let doc = Document::from_str("test.md", content).unwrap();
    let config = Config {
      sections: Some(vec![S("one"), S("three")]),
      ..Config::default()
    };
    let mut issues = vec![];
    super::scan(&doc, &config, &mut issues);
    let want = vec![];
    assert_eq!(issues, want);
  }

  #[test]
  fn empty() {
    let content = indoc! {"
            # Test
            ### one
            text
            ### two
            text
            ### three
            text"};
    let doc = Document::from_str("test.md", content).unwrap();
    let config = Config {
      sections: None,
      ..Config::default()
    };
    let mut issues = vec![];
    super::scan(&doc, &config, &mut issues);
    let want = vec![];
    assert_eq!(issues, want);
  }
}
