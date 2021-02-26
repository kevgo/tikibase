pub struct Section {
  pub title: Line,
  pub body: Vec<Line>,
}

pub struct Line {
  /// The line number relative to the section title line, 0-based.
  pub line_number: u32,
  pub text: String,
}
