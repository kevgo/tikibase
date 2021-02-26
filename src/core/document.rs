use super::section;
use super::section::Section;
use std::borrow::Cow;
use std::convert::TryInto;
use std::io::BufRead;
use std::path::PathBuf;

pub struct Document {
  pub path: PathBuf,
  pub sections: Vec<Section>,
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
  // NOTE: modeling the lines argument this way to allow iterating over
  // std::io::Lines in production efficiently (without additional allocations)
  // and std::str::Lines in tests.
  // See https://stackoverflow.com/a/37029631/1363753.
  let mut sections: Vec<Section> = Vec::new();
  let mut section_builder = section::empty_builder();
  for (line_number, line) in lines.into_iter().enumerate() {
    let line = line.into().into_owned();
    if line.starts_with('#') {
      if let Some(section) = section_builder.result() {
        sections.push(section);
      }
      section_builder = section::builder_with_title_line(line, line_number.try_into().unwrap());
    } else {
      section_builder.add_body_line(line);
    }
  }
  if let Some(section) = section_builder.result() {
    sections.push(section);
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
