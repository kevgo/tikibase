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
    pub fn from_str<P: Into<PathBuf>>(path: P, text: &str) -> Result<Document, String> {
        Document::from_lines(text.lines().map(|line| line.to_string()), path.into())
    }

    /// persists the changes made to this document to disk
    pub fn flush(&self, root: &Path) {
        let mut file = std::fs::File::create(root.join(&self.path)).unwrap();
        file.write_all(self.text().as_bytes()).unwrap();
    }

    /// provides the last section in this document
    pub fn last_section_mut(&mut self) -> &mut Section {
        match self.content_sections.len() {
            0 => &mut self.title_section,
            index => self.content_sections.get_mut(index - 1).unwrap(),
        }
    }

    /// provides the number of lines in this document
    pub fn lines_count(&self) -> u32 {
        match self.content_sections.len() {
            0 => self.title_section.last_line_abs(),
            cnt => self.content_sections[cnt - 1].last_line_abs(),
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

    /// provides the section types in this document
    pub fn section_types(&self) -> Vec<String> {
        self.content_sections
            .iter()
            .map(|section| section.section_type())
            .collect()
    }

    /// provides the complete textual content of this document
    pub fn text(&self) -> String {
        let mut result = self.title_section.text();
        for section in &self.content_sections {
            result.push_str(&section.text());
        }
        result
    }

    /// provides the human-readable title of this document
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

    #[test]
    fn from_str_valid() {
        let content = "\
# test
### section 1
content";
        let doc = Document::from_str("one.md", content).unwrap();
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
        match Document::from_str("one.md", "content") {
            Err(e) => assert_eq!(e, "one.md  no title section"),
            Ok(_) => panic!(),
        }
    }

    mod lines_count {

        use super::super::Document;

        #[test]
        fn with_content_sections() {
            let give = "\
# Title
title text
### Section 1
one
two
### Section 2
foo
";
            let doc = Document::from_str("test.md", give).unwrap();
            assert_eq!(doc.lines_count(), 6);
        }

        #[test]
        fn no_content_sections() {
            let give = "\
# Title
title text
";
            let doc = Document::from_str("test.md", give).unwrap();
            assert_eq!(doc.lines_count(), 1);
        }
    }

    mod last_section_mut {

        use super::Document;

        #[test]
        fn has_content_section() {
            let give = "\
# Title
title text

### s1

text
";
            let mut doc = Document::from_str("test.md", give).unwrap();
            let have = doc.last_section_mut();
            assert_eq!(have.title_line.text, "### s1");
        }

        #[test]
        fn no_content_sections() {
            let give = "\
# Title
title text
";
            let mut doc = Document::from_str("test.md", give).unwrap();
            let have = doc.last_section_mut();
            assert_eq!(have.title_line.text, "# Title");
        }
    }

    #[test]
    fn section_types() {
        let content = "\
# Title
title text
### Section 1
two
### Section 2
foo
";
        let doc = Document::from_str("test.md", content).unwrap();
        let have = doc.section_types();
        let want = vec!["Section 1".to_string(), "Section 2".to_string()];
        assert_eq!(have, want);
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
        let doc = Document::from_str("test.md", give).unwrap();
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
        let doc = Document::from_str("test.md", give).unwrap();
        let have = doc.title();
        assert_eq!(have, "Title");
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
    valid: bool,
}

/// Provides a builder instance loaded with the given title line.
pub fn builder_with_title_line<S: Into<String>>(text: S, line_number: u32) -> SectionBuilder {
    SectionBuilder {
        title_line: text.into(),
        line_number,
        body: Vec::new(),
        valid: true,
    }
}

/// Null value for SectionBuilder instances
pub fn placeholder_builder() -> SectionBuilder {
    SectionBuilder {
        title_line: "".to_string(),
        line_number: 0,
        body: Vec::new(),
        valid: false,
    }
}

impl SectionBuilder {
    pub fn add_body_line<S: Into<String>>(&mut self, line: S) {
        if !self.valid {
            panic!("cannot add to an invalid builder");
        }
        self.body.push(Line { text: line.into() });
    }

    /// Provides the content this builder has accumulated.
    pub fn result(self) -> Option<Section> {
        match self.valid {
            false => None,
            true => Some(Section {
                title_line: Line {
                    text: self.title_line,
                },
                line_number: self.line_number,
                body: self.body,
            }),
        }
    }
}
