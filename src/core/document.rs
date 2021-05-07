use super::line::Line;
use super::section::Section;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub struct Document {
    /// the path relative to the Tikibase root directory
    pub path: PathBuf,
    pub title_section: Section,
    pub content_sections: Vec<Section>,
}

impl Document {
    /// provides a Document instance containing the given text
    pub fn from_lines<T>(lines: T, path: PathBuf) -> Result<Document, String>
    where
        T: Iterator<Item = String>,
    {
        let mut sections: Vec<Section> = Vec::new();
        let mut section_builder = placeholder_builder();
        for (line, line_number) in lines.zip(0..) {
            if line.starts_with('#') {
                if let Some(section) = section_builder.result() {
                    sections.push(section);
                }
                section_builder = builder_with_title_line(line, line_number);
            } else if section_builder.valid {
                section_builder.add_body_line(line);
            } else {
                return Err(format!("{}  no title section", path.to_string_lossy()));
            }
        }
        if let Some(section) = section_builder.result() {
            sections.push(section);
        }
        let content_sections = sections.split_off(1);
        Ok(Document {
            path,
            title_section: sections.pop().unwrap(),
            content_sections,
        })
    }

    /// provides a Document instance containing the content of the file at the given path
    pub fn from_str(path: PathBuf, text: &str) -> Result<Document, String> {
        Document::from_lines(text.lines().map(|line| line.to_string()), path)
    }

    /// persists the changes made to this document to disk
    pub fn flush(&self, root: &Path) {
        let mut file = std::fs::File::create(root.join(&self.path)).unwrap();
        file.write_all(self.text().as_bytes()).unwrap();
    }

    pub fn last_line(&self) -> u32 {
        let sections_count = self.content_sections.len();
        if sections_count == 0 {
            sections_count as u32
        } else {
            self.content_sections[sections_count - 1].last_line()
        }
    }

    /// provides a non-consuming iterator for all sections in this document
    pub fn sections(&self) -> SectionIterator {
        SectionIterator {
            title_section: &self.title_section,
            body_iter: self.content_sections.iter(),
            emitted_title: false,
        }
    }

    /// provides the complete textual content of this document
    pub fn text(&self) -> String {
        let mut result = self.title_section.text();
        for section in &self.content_sections {
            result.push_str(&section.text());
        }
        result
    }

    pub fn title(&self) -> String {
        self.title_section.section_type()
    }
}

/// iterates all sections of a document
pub struct SectionIterator<'a> {
    title_section: &'a Section,
    body_iter: std::slice::Iter<'a, Section>,
    emitted_title: bool,
}

impl<'a> Iterator for SectionIterator<'a> {
    type Item = &'a Section;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.emitted_title {
            self.emitted_title = true;
            Some(self.title_section)
        } else {
            self.body_iter.next()
        }
    }
}

// -------------------------------------------------------------------------------------
// TESTS
// -------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {

    use super::Document;
    use std::path::PathBuf;

    #[test]
    fn from_str_valid() {
        let content = "\
# test
### section 1
content";
        let doc = Document::from_str(PathBuf::from("one.md"), content).unwrap();
        let mut sections = doc.sections();
        match sections.next() {
            None => panic!("expected title section"),
            Some(s1) => assert_eq!(s1.title_line.text, "# test"),
        }
        match sections.next() {
            None => panic!("expected s1"),
            Some(s1) => assert_eq!(s1.title_line.text, "### section 1"),
        }
        match sections.next() {
            None => return,
            Some(_) => panic!("unexpected section"),
        }
    }

    #[test]
    fn from_str_invalid() {
        match Document::from_str(PathBuf::from("one.md"), "content") {
            Err(e) => assert_eq!(e, "one.md  no title section"),
            Ok(_) => panic!(),
        }
    }

    #[test]
    fn last_line() {
        let give = "\
# Title
title text
### Section 1
one
two
### Section 2
foo
";
        let doc = Document::from_str(PathBuf::from("test.md"), give).unwrap();
        assert_eq!(doc.last_line(), 6);
    }

    #[test]
    fn text() {
        let give = "\
# Title
title text
### Section 1
one
two
### Section 2
foo
";
        let doc = Document::from_str(PathBuf::from("test.md"), give).unwrap();
        let have = doc.text();
        assert_eq!(have, give);
    }

    #[test]
    fn title() {
        let give = "\
# Title
title text
### Section 1
one
";
        let doc = Document::from_str(PathBuf::from("test.md"), give).unwrap();
        let have = doc.title();
        assert_eq!(have, give);
    }
}

// -------------------------------------------------------------------------------------
// HELPERS
// -------------------------------------------------------------------------------------

/// Allows building up sections one line at a time.
pub struct SectionBuilder {
    pub line_number: u32,
    title_line: String,
    body: Vec<Line>,
    body_line_number: u32,
    valid: bool,
}

/// Provides a builder instance loaded with the given title line.
pub fn builder_with_title_line(text: String, line_number: u32) -> SectionBuilder {
    SectionBuilder {
        title_line: text,
        line_number,
        body: Vec::new(),
        body_line_number: 0,
        valid: true,
    }
}

/// Null value for SectionBuilder instances
pub fn placeholder_builder() -> SectionBuilder {
    SectionBuilder {
        title_line: "".to_string(),
        line_number: 0,
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
            section_offset: self.body_line_number,
            text: line,
        });
    }

    /// Provides the content this builder has accumulated.
    pub fn result(self) -> Option<Section> {
        match self.valid {
            false => None,
            true => Some(Section {
                title_line: Line {
                    text: self.title_line,
                    section_offset: 0,
                },
                line_number: self.line_number,
                body: self.body,
            }),
        }
    }
}
