use std::borrow::Cow;
use std::convert::TryInto;
use std::io::BufRead;
use std::path::PathBuf;

pub struct Document {
  pub path: PathBuf,
  pub sections: Vec<Section>,
}

pub struct Section {
  pub title: Line,
  pub body: Vec<Line>,
}

pub struct Line {
  /// The line number relative to the section title line, 0-based.
  pub line_number: u32,
  pub text: String,
}

pub fn load(path: PathBuf) -> Document {
  let file = std::fs::File::open(&path).unwrap();
  new(
    path,
    std::io::BufReader::new(file).lines().map(|r| r.unwrap()),
  )
}

pub fn new<'a, I>(path: PathBuf, lines: I) -> Document
where
  I: IntoIterator,
  I::Item: Into<Cow<'a, str>>,
{
  let mut sections: Vec<Section> = Vec::new();
  let mut title = Line {
    text: "".to_string(),
    line_number: 0,
  };
  let mut body: Vec<Line> = Vec::new();
  let mut section_line_number: u32 = 0;
  for (line_number, line) in lines.into_iter().enumerate() {
    let line = line.into().into_owned();
    if line.starts_with('#') {
      // beginning of a new section
      if !title.text.is_empty() {
        // we have collected a section with content --> store it
        sections.push(Section { title, body });
      }
      // store the new section title
      title = Line {
        text: line,
        line_number: line_number.try_into().unwrap(),
      };
      section_line_number = 0;
      body = Vec::new();
    } else {
      section_line_number += 1;
      body.push(Line {
        line_number: section_line_number,
        text: line,
      });
    }
  }
  if !title.text.is_empty() {
    sections.push(Section { title, body });
  }
  Document { path, sections }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new() {
    let content: &'static str = "\
# Title
title text
### Section 1
one
two
### Section 2
foo
";
    let path = PathBuf::new();
    let have = super::new(path.clone(), content.lines());
    assert_eq!(have.path, path);
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
