use std::io::BufRead;
use std::path::PathBuf;

pub struct Document {
  path: PathBuf,
  sections: Vec<Section>,
}

pub struct Section {
  title: Line,
  body: Vec<Line>,
}

pub struct Line {
  number: u32,
  text: String,
}

pub fn new(path: PathBuf) -> Document {
  let file = std::fs::File::open(&path).unwrap();
  let mut sections: Vec<Section> = Vec::new();
  let mut title = "".to_string();
  let mut body: Vec<Line> = Vec::new();
  let mut number: u32 = 0;
  for line in std::io::BufReader::new(file).lines() {
    number += 1;
    let line = line.unwrap();
    if line.starts_with('#') {
      if title.is_empty() {
        sections.push(Section {
          title: Line {
            number,
            text: title,
          },
          body,
        });
      }
      title = line;
      body = Vec::new();
    } else {
      body.push(Line { number, text: line });
    }
  }
  if title != "" {
    sections.push(Section {
      title: Line {
        number,
        text: title,
      },
      body,
    });
  }
  Document { path, sections }
}
