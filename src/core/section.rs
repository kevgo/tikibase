use super::line::Line;
use heck::KebabCase;

pub struct Section {
    /// The line number at which this section starts, 0-based.
    pub line_number: u32,
    /// Complete textual content of this section's title line, e.g. "# Title"
    pub title_line: Line,
    /// Optional content of this section
    pub body: Vec<Line>,
}

impl Section {
    /// provides an link anchor for this section, in GitHub format
    pub fn anchor(&self) -> String {
        format!("#{}", self.section_type().to_kebab_case())
    }

    /// provides a non-consuming iterator for all lines in this section
    pub fn lines(&self) -> LinesIterator {
        LinesIterator {
            title_line: &self.title_line,
            body_iter: self.body.iter(),
            emitted_title: false,
        }
    }

    /// provides the absolute line number of the last line in this section
    pub fn last_line_abs(&self) -> u32 {
        self.line_number + (self.body.len() as u32)
    }

    /// adds a new line with the given text to this section
    pub fn push_line<S: Into<String>>(&mut self, text: S) {
        self.body.push(Line { text: text.into() });
    }

    // TODO: return a string slice here
    pub fn section_type(&self) -> &str {
        for (i, c) in self.title_line.text.char_indices() {
            if c != '#' && c != ' ' {
                return &self.title_line.text[i..];
            }
        }
        &self.title_line.text[..]
    }

    /// provides the complete text of this section
    pub fn text(&self) -> String {
        let mut result = self.title_line.text.clone();
        result.push('\n');
        for line in &self.body {
            result.push_str(&line.text);
            result.push('\n');
        }
        result
    }
}

impl Default for Section {
    fn default() -> Self {
        Section {
            line_number: 0,
            title_line: Line {
                text: "### section".into(),
            },
            body: vec![],
        }
    }
}

/// an iterator for Lines
pub struct LinesIterator<'a> {
    title_line: &'a Line,
    body_iter: std::slice::Iter<'a, Line>,
    emitted_title: bool,
}

impl<'a> Iterator for LinesIterator<'a> {
    type Item = &'a Line;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.emitted_title {
            self.emitted_title = true;
            Some(self.title_line)
        } else {
            self.body_iter.next()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::document::Document;
    use super::*;
    use crate::testhelpers::{line_with_text, section_with_title};

    #[test]
    fn anchor() {
        let tests = vec![
            ("foo", "#foo"),
            ("what is it", "#what-is-it"),
            ("A Complex Section", "#a-complex-section"),
        ];
        for (give, want) in tests.into_iter() {
            let section = section_with_title(give);
            assert_eq!(section.anchor(), want);
        }
    }

    mod last_line {

        use crate::core::section::Section;
        use crate::testhelpers::line_with_text;
        use std::default::Default;

        #[test]
        fn no_body() {
            let section = Section {
                line_number: 12,
                ..Default::default()
            };
            assert_eq!(section.last_line_abs(), 12);
        }

        #[test]
        fn with_body() {
            let section = Section {
                line_number: 12,
                body: vec![line_with_text("")],
                ..Default::default()
            };
            assert_eq!(section.last_line_abs(), 13);
        }
    }

    #[test]
    fn lines() {
        let content = "\
# test
title content";
        let doc = Document::from_str("foo", content).unwrap();
        let mut lines = doc.title_section.lines();
        match lines.next() {
            None => panic!("expected title line"),
            Some(line) => assert_eq!(line.text, "# test"),
        }
        match lines.next() {
            None => panic!("expected body line 1"),
            Some(line) => assert_eq!(line.text, "title content"),
        }
        match lines.next() {
            None => return,
            Some(_) => panic!("unexpected line"),
        }
    }

    mod push_line {
        use crate::core::section::Section;
        use crate::testhelpers::line_with_text;

        #[test]
        fn no_body() {
            let mut section = Section {
                body: vec![],
                ..Default::default()
            };
            section.push_line("new line");
            assert_eq!(section.body.len(), 1);
            assert_eq!(section.body[0].text, "new line");
        }

        #[test]
        fn with_body() {
            let mut section = Section {
                body: vec![line_with_text("l1")],
                ..Default::default()
            };
            section.push_line("new line");
            assert_eq!(section.body.len(), 2);
            assert_eq!(section.body[0].text, "l1");
            assert_eq!(section.body[1].text, "new line");
        }
    }

    #[test]
    fn section_type() {
        let tests = vec![
            ("# Title", "Title"),
            ("### Title", "Title"),
            ("Title", "Title"),
            ("###", ""),
        ];
        for (title, want) in tests.into_iter() {
            let section = section_with_title(title);
            let have = section.section_type();
            assert_eq!(have, want);
        }
    }

    #[test]
    fn text() {
        let section = Section {
            title_line: line_with_text("### welcome"),
            body: vec![line_with_text(""), line_with_text("content")],
            ..Default::default()
        };
        assert_eq!(section.text(), "### welcome\n\ncontent\n");
    }
}
