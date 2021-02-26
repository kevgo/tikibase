pub struct Section {
  /// The line number at which this section starts, 0-based.
  pub line_number: u32,
  /// Complete textual content of this section's title line, e.g. "# Title"
  pub title_line: String,
  /// Optional content of this section
  pub body: Vec<Line>,
}

pub struct Line {
  /// The line number relative to the section title line, 0-based.
  pub line_number: u32,
  pub text: String,
}
