use super::section::{Line, Section};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub struct Document {
  pub path: PathBuf,
  pub sections: Vec<Section>,
}

pub fn load(path: PathBuf) -> Document {
  let mut sections: Vec<Section> = Vec::new();
  let mut section_builder = placeholder_builder();
  let file = File::open(&path).unwrap();
  for (line, line_number) in BufReader::new(file).lines().into_iter().zip(0..) {
    let line = line.unwrap();
    if line.starts_with('#') {
      if let Some(section) = section_builder.result() {
        sections.push(section);
      }
      section_builder = builder_with_title_line(line, line_number);
    } else {
      section_builder.add_body_line(line);
    }
  }
  if let Some(section) = section_builder.result() {
    sections.push(section);
  }
  Document { path, sections }
}

// -------------------------------------------------------------------------------------
// TESTS
// -------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {

  #[test]
  fn load() {
    let content = "\
# Title
title text
### Section 1
one
two
### Section 2
foo
";
    let tmp_dir = tempfile::tempdir().unwrap();
    let file_path = tmp_dir.path().join("file.md");
    std::fs::write(&file_path, content).unwrap();
    let have = super::load(file_path);
    assert_eq!(have.sections.len(), 3);
    assert_eq!(have.sections[0].title.text, "# Title");
    assert_eq!(have.sections[0].title.line_number, 0);
    assert_eq!(have.sections[0].body.len(), 1);
    assert_eq!(have.sections[0].body[0].text, "title text");
    assert_eq!(have.sections[0].body[0].line_number, 1);
    assert_eq!(have.sections[1].title.text, "### Section 1");
    assert_eq!(have.sections[1].title.line_number, 2);
    assert_eq!(have.sections[1].body.len(), 2);
    assert_eq!(have.sections[1].body[0].text, "one");
    assert_eq!(have.sections[1].body[0].line_number, 1);
    assert_eq!(have.sections[1].body[1].text, "two");
    assert_eq!(have.sections[1].body[1].line_number, 2);
    assert_eq!(have.sections[2].title.text, "### Section 2");
    assert_eq!(have.sections[2].title.line_number, 5);
    assert_eq!(have.sections[2].body.len(), 1);
    assert_eq!(have.sections[2].body[0].text, "foo");
    assert_eq!(have.sections[2].body[0].line_number, 1);
  }
}

// -------------------------------------------------------------------------------------
// HELPERS
// -------------------------------------------------------------------------------------

/// Allows building up sections one line at a time.
pub struct SectionBuilder {
  title: Line,
  body: Vec<Line>,
  body_line_number: u32,
  valid: bool,
}

/// Provides a builder instance loaded with the given title line.
pub fn builder_with_title_line(text: String, number: u32) -> SectionBuilder {
  SectionBuilder {
    title: Line {
      text,
      line_number: number,
    },
    body: Vec::new(),
    body_line_number: 0,
    valid: true,
  }
}

/// Null value for SectionBuilder instances
pub fn placeholder_builder() -> SectionBuilder {
  SectionBuilder {
    title: Line {
      text: "".to_string(),
      line_number: 0,
    },
    body: Vec::new(),
    body_line_number: 0,
    valid: false,
  }
}

impl SectionBuilder {
  pub fn add_body_line(&mut self, line: String) {
    if !self.valid {
      panic!("cannot add to an invalid builder");
    }
    self.body_line_number += 1;
    self.body.push(Line {
      line_number: self.body_line_number,
      text: line,
    });
  }

  /// Provides the content this builder has accumulated.
  pub fn result(self) -> Option<Section> {
    match self.valid {
      false => None,
      true => Some(Section {
        title: self.title,
        body: self.body,
      }),
    }
  }
}
