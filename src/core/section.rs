use std::path::PathBuf;

pub struct Section<'a> {
  /// The path of the document that contains this section.
  pub path: &'a PathBuf,
  /// The line number at which this section starts, 0-based.
  pub line_number: u32,
  /// Complete textual content of this section's title line, e.g. "# Title"
  pub title_line: String,
  /// Optional content of this section
  pub body: Vec<Line>,
}

impl Section<'a> {
  pub fn section_type(&self) -> String {
    let pos = self
      .title_line
      .char_indices()
      .find(|p| p.1 != '#' && p.1 != ' ');
    match pos {
      None => "".to_string(),
      Some(pos) => self.title_line.clone().split_off(pos.0),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn section_type() {
    let tests = vec![
      ("# Title", "Title"),
      ("### Title", "Title"),
      ("Title", "Title"),
      ("###", ""),
    ];
    for (give, want) in tests.into_iter() {
      let section = Section {
        line_number: 2,
        title_line: give.to_string(),
        body: vec![],
        path: &PathBuf::new(),
      };
      let have = section.section_type();
      assert_eq!(have, want.to_string(), "want: '{}', have: '{}'", want, have);
    }
  }
}

pub struct Line {
  /// The line number relative to the section title line, 0-based.
  pub line_number: u32,
  pub text: String,
}
