use super::{Image, Line, Link};
use heck::ToKebabCase;

/// a section in a document, from one heading to above the next heading
#[derive(Debug, Default, Eq, Hash, PartialEq)]
pub struct Section {
  /// the line number at which this section starts, 0-based
  pub line_number: u32,

  /// complete textual content of this section's title line, e.g. "# Title"
  pub title_line: Line,

  /// optional content of this section
  pub body: Vec<Line>,

  /// the cursor column at which the title text starts (derived value),
  /// counterpart to `end`
  pub title_text_start: usize,

  /// cache for the heading level (h1-h6)
  pub level: u8,
}

impl Section {
  /// provides the link anchor for this section, in GitHub format
  pub fn anchor(&self) -> String {
    format!("#{}", self.human_title().to_kebab_case())
  }

  /// indicates whether this section contains no content
  pub fn is_empty(&self) -> bool {
    self.body.iter().all(|line| line.text.is_empty())
  }

  /// provides the cursor column at which the title text ends,
  /// counterpart to `start`
  pub fn title_text_end(&self) -> u32 {
    self.title_line.text.len() as u32
  }

  /// provides a human-readable version of this section's title, e.g. "Hello" for a section with the title "# Hello"
  pub fn human_title(&self) -> &str {
    &self.title_line.text[self.title_text_start..]
  }

  /// returns the last line of this section
  pub fn last_line(&self) -> &Line {
    match self.body.last() {
      Some(last_body_line) => last_body_line,
      None => &self.title_line,
    }
  }

  /// provides the absolute line number of the last line in this section
  pub fn last_line_abs(&self) -> u32 {
    self.line_number + (self.body.len() as u32)
  }

  /// provides a non-consuming iterator for all lines in this section
  pub fn lines(&self) -> LinesIterator {
    LinesIterator {
      title_line: &self.title_line,
      body_iter: self.body.iter(),
      emitted_title: false,
    }
  }

  pub fn new<IS: Into<String>>(line_number: u32, title: IS, body: Vec<IS>) -> Self {
    let title: String = title.into();
    let (level, start) = Self::parse_title(&title);
    Self {
      line_number,
      title_line: Line::from(title),
      title_text_start: start,
      level,
      body: body.into_iter().map(Line::from).collect(),
    }
  }

  /// provides the level and start position of the human title portion within the given section title
  pub fn parse_title(title: &str) -> (u8, usize) {
    let mut chars = title.char_indices();
    let mut level = 0;
    for (i, c) in chars.by_ref() {
      if c != '#' {
        level = i;
        break;
      }
    }
    if level == 0 {
      level = title.len();
    }
    let mut start = level;
    for (i, c) in chars {
      if c != ' ' {
        start = i;
        break;
      }
    }
    if start == level {
      start = title.len();
    }
    (level as u8, start)
  }

  /// adds a new line with the given text to this section
  pub fn push_line(&mut self, text: impl Into<String>) {
    self.body.push(Line::from(text));
  }

  /// populates the given accumulator with all references in this section
  pub fn references(&self, links: &mut Vec<Link>, images: &mut Vec<Image>) {
    self.title_line.references(self.line_number, links, images);
    for (i, line) in self.body.iter().enumerate() {
      line.references(self.line_number + i as u32 + 1, links, images);
    }
  }

  #[cfg(test)]
  fn scaffold() -> Self {
    Self::new(0, "### section", vec![])
  }

  /// provides the complete text of this section
  pub fn text(&self) -> String {
    let mut result = self.title_line.text.to_string();
    result.push('\n');
    for line in &self.body {
      result.push_str(&line.text);
      result.push('\n');
    }
    result
  }

  /// provides a section with the given title
  #[cfg(test)]
  pub fn with_body(body: Vec<&str>) -> Self {
    Self::new(0, "# title", body)
  }

  /// provides a section with the given title
  #[cfg(test)]
  pub fn with_title(title: &str) -> Self {
    Self::new(0, title, vec![])
  }
}

/// an iterator for Lines
pub struct LinesIterator<'a> {
  title_line: &'a Line,
  body_iter: std::slice::Iter<'a, Line>,
  emitted_title: bool,
}

impl<'a> Iterator for LinesIterator<'a> {
  type Item = &'a Line;

  fn next(&mut self) -> Option<Self::Item> {
    if !self.emitted_title {
      self.emitted_title = true;
      return Some(self.title_line);
    }
    self.body_iter.next()
  }
}

/// allows building up sections one line at a time
pub struct Builder {
  pub line_number: u32,
  title_line: String,
  body: Vec<String>,
}

impl Builder {
  /// Provides a builder instance loaded with the given title line.
  pub fn new(title: impl Into<String>, line_number: u32) -> Builder {
    Builder {
      title_line: title.into(),
      line_number,
      body: Vec::new(),
    }
  }

  pub fn add_line(&mut self, text: impl Into<String>) {
    self.body.push(text.into());
  }

  /// Provides the content this builder has accumulated.
  pub fn result(self) -> Section {
    Section::new(self.line_number, self.title_line, self.body)
  }
}

#[cfg(test)]
mod tests {
  use super::super::document::Document;
  use super::*;
  use indoc::indoc;

  #[test]
  fn anchor() {
    let tests = vec![
      ("### foo", "#foo"),
      ("### A Complex Section", "#a-complex-section"),
    ];
    for (give, want) in tests {
      let section = Section::with_title(give);
      assert_eq!(section.anchor(), want);
    }
  }

  mod is_empty {
    use crate::database::{Line, Section};

    #[test]
    fn completely_empty() {
      let section = Section {
        body: vec![],
        ..Section::default()
      };
      assert!(section.is_empty());
    }

    #[test]
    fn empty_lines() {
      let section = Section {
        body: vec![Line::from(""), Line::from("")],
        ..Section::default()
      };
      assert!(section.is_empty());
    }

    #[test]
    fn with_content() {
      let section = Section {
        body: vec![Line::from("text"), Line::from("")],
        ..Section::default()
      };
      assert!(!section.is_empty());
    }
  }

  #[test]
  fn human_title() {
    let tests = vec![("# title", "title"), ("###### title", "title"), ("###", "")];
    for (give, want) in tests {
      let section = Section::with_title(give);
      assert_eq!(section.human_title(), want);
    }
  }

  mod last_line {
    use crate::database::{Line, Section};

    #[test]
    fn has_body() {
      let section = Section::with_body(vec!["one", "two"]);
      let have = section.last_line();
      let want = Line::from("two");
      assert_eq!(have, &want);
    }

    #[test]
    fn no_body() {
      let section = Section::new(0, "### title", vec![]);
      let have = section.last_line();
      let want = Line::from("### title");
      assert_eq!(have, &want);
    }
  }

  mod last_line_abs {
    use crate::database::{Line, Section};

    #[test]
    fn no_body() {
      let section = Section {
        line_number: 12,
        ..Section::scaffold()
      };
      assert_eq!(section.last_line_abs(), 12);
    }

    #[test]
    fn with_body() {
      let section = Section {
        line_number: 12,
        body: vec![Line::from("")],
        ..Section::scaffold()
      };
      assert_eq!(section.last_line_abs(), 13);
    }
  }

  #[test]
  fn level() {
    let tests = vec![("# title", 1), ("###### title", 6), ("###", 3)];
    for (give, want) in tests {
      let section = Section::with_title(give);
      assert_eq!(section.level, want);
    }
  }

  #[test]
  fn lines() {
    let give = indoc! {"
            # test
            title content"};
    let doc = Document::from_str("foo", give).unwrap();
    let mut have = doc.title_section.lines();
    let line = have.next().expect("expected title line");
    assert_eq!(line.text, "# test");
    let line = have.next().expect("expected body line 1");
    assert_eq!(line.text, "title content");
    assert!(have.next().is_none(), "unexpected line");
  }

  mod parse_title {
    use crate::database::Section;

    #[test]
    fn normal() {
      let give = "### title";
      let want = (3, 4);
      let have = Section::parse_title(give);
      assert_eq!(have, want);
    }

    #[test]
    fn extra_space() {
      let give = "###   title";
      let want = (3, 6);
      let have = Section::parse_title(give);
      assert_eq!(have, want);
    }

    #[test]
    fn missing_title() {
      let give = "###   ";
      let want = (3, 6);
      let have = Section::parse_title(give);
      assert_eq!(have, want);
    }
  }

  mod push_line {
    use crate::database::{Line, Section};

    #[test]
    fn no_body() {
      let mut have = Section {
        body: vec![],
        ..Section::scaffold()
      };
      have.push_line("new line");
      let want = Section {
        body: vec![Line::from("new line")],
        ..Section::scaffold()
      };
      pretty::assert_eq!(have, want);
    }

    #[test]
    fn with_existing_body() {
      let mut section = Section {
        body: vec![Line::from("l1")],
        ..Section::scaffold()
      };
      section.push_line("new line");
      let have = section.body;
      let want = vec![Line::from("l1"), Line::from("new line")];
      pretty::assert_eq!(have, want);
    }
  }

  #[test]
  fn text() {
    let section = Section::new(0, "### welcome", vec!["", "content"]);
    assert_eq!(section.text(), "### welcome\n\ncontent\n");
  }
}
