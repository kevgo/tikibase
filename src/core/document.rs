use super::error::UserError;
use super::line::Line;
use super::section::Section;
use std::fs::File;
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
    pub fn from_lines<T>(lines: T, path: PathBuf) -> Result<Document, UserError>
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
            } else if !section_builder.add_body_line(line) {
                return Err(UserError(format!(
                    "\"{}\"  has no title section",
                    path.to_string_lossy()
                )));
            }
        }
        if let Some(section) = section_builder.result() {
            sections.push(section);
        }
        let content_sections = sections.split_off(1);
        let title_section = sections.pop().ok_or_else(|| {
            UserError(format!(
                "\"{}\" has no title section",
                path.to_string_lossy()
            ))
        })?;
        Ok(Document {
            path,
            title_section,
            content_sections,
        })
    }

    /// provides a Document instance containing the content of the file at the given path
    pub fn from_str(path: PathBuf, text: &str) -> Result<Document, UserError> {
        Document::from_lines(text.lines().map(|line| line.to_string()), path)
    }

    /// persists the changes made to this document to disk
    pub fn flush(&self, root: &Path) -> Result<(), UserError> {
        let filename = root.join(&self.path);
        let mut file = File::create(&filename).map_err(|_| {
            UserError(format!(
                "cannot create file \"{}\"",
                filename.to_string_lossy()
            ))
        })?;
        file.write_all(self.text().as_bytes()).map_err(|_| {
            UserError(format!(
                "cannot write file \"{}\"",
                filename.to_string_lossy()
            ))
        })?;
        Ok(())
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
    fn sections() {
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
}

// -------------------------------------------------------------------------------------
// HELPERS
// -------------------------------------------------------------------------------------

/// Allows building up sections one line at a time.
pub struct SectionBuilder {
    line_number: u32,
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
    pub fn add_body_line(&mut self, line: String) -> bool {
        if !self.valid {
            return false;
        }
        self.body_line_number += 1;
        self.body.push(Line {
            section_offset: self.body_line_number,
            text: line,
        });
        true
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
