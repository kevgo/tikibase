use std::io::BufRead;
use std::path::PathBuf;

pub struct Document {
  pub path: PathBuf,
  pub sections: Vec<Section>,
}

pub struct Section {
  pub title: Line,
  pub body: Vec<Line>,
  /// The line number where this section starts, 0-based
  pub line_number: u32,
}

pub struct Line {
  /// The line number relative to the section start, 0-based.
  pub line_number: u32,
  pub text: String,
}

pub fn new(path: PathBuf) -> Document {
  let file = std::fs::File::open(&path).unwrap();
  let mut sections: Vec<Section> = Vec::new();
  let mut title = "".to_string();
  let mut body: Vec<Line> = Vec::new();
  let mut line_number: u32 = 0;
  for line in std::io::BufReader::new(file).lines() {
    let line = line.unwrap();
    if line.starts_with('#') {
      if !title.is_empty() {
        sections.push(Section {
          title: Line {
            line_number,
            text: title,
          },
          body,
          line_number,
        });
      }
      title = line;
      body = Vec::new();
    } else {
      body.push(Line {
        line_number,
        text: line,
      });
    }
    line_number += 1;
  }
  if !title.is_empty() {
    sections.push(Section {
      title: Line {
        line_number,
        text: title,
      },
      body,
      line_number,
    });
  }
  Document { path, sections }
}
