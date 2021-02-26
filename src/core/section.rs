pub struct Section {
  pub title: Line,
  pub body: Vec<Line>,
}

/// Allows building up sections one line at a time.
pub struct Builder {
  title: Line,
  body: Vec<Line>,
  body_line_number: u32,
  /// Indicates whethen this builder contains content that should be used.
  valid: bool,
}

/// Provides a builder instance loaded with the given title line.
pub fn builder_with_title_line(text: String, number: u32) -> Builder {
  Builder {
    title: Line {
      text,
      line_number: number,
    },
    body: Vec::new(),
    body_line_number: 0,
    valid: true,
  }
}

/// Provides an empty (placeholder) builder instance.
pub fn empty_builder() -> Builder {
  Builder {
    title: Line {
      text: "".to_string(),
      line_number: 0,
    },
    body: Vec::new(),
    body_line_number: 0,
    valid: false,
  }
}

impl Builder {
  pub fn add_body_line(&mut self, line: String) {
    self.body_line_number += 1;
    self.body.push(Line {
      line_number: self.body_line_number,
      text: line,
    });
  }

  /// Provides the received content of this builder.
  pub fn result(self) -> Option<Section> {
    match self.valid {
      true => Some(Section {
        title: self.title,
        body: self.body,
      }),
      false => None,
    }
  }
}

pub struct Line {
  /// The line number relative to the section title line, 0-based.
  pub line_number: u32,
  pub text: String,
}
