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
  let file = std::fs::File::open(path).unwrap();
  let sections: Vec<Section> = Vec::new();
  let mut title = "";
  let mut body: Vec<Line> = Vec::new();
  let mut number: u32 = 0;
  for line in std::io::BufReader::new(file).lines() {
    number += 1;
    if line.starts_with('#') {
      if title != "" {
        sections.push(Section { title: Line{, body });
      }
      title = "";
      body = Vec::new();
    }
  }
  if title != "" {
    sections.push(Section { title, body });
  }
  let doc = Document { /*path*/ };
  doc
}
