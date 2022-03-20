use super::Line;
use heck::ToKebabCase;

/// a section in a document, from one heading to above the next heading
#[derive(Debug, PartialEq)]
pub struct Section {
    /// the line number at which this section starts, 0-based
    pub line_number: u32,
    /// complete textual content of this section's title line, e.g. "# Title"
    pub title_line: Line,
    /// optional content of this section
    pub body: Vec<Line>,
}

impl Section {
    /// provides the link anchor for this section, in GitHub format
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
        self.body.push(Line::new(text));
    }

    #[cfg(test)]
    fn scaffold() -> Self {
        Section {
            line_number: 0,
            title_line: Line::new("### section"),
            body: Vec::new(),
        }
    }

    /// provides a human-readable description of this section, e.g. "Hello" for a section with the title "# Hello"
    pub fn section_type(&self) -> &str {
        for (i, c) in self.title_line.text().char_indices() {
            if c != '#' && c != ' ' {
                return &self.title_line.text()[i..];
            }
        }
        ""
    }

    /// provides the complete text of this section
    pub fn text(&self) -> String {
        let mut result = self.title_line.text().to_string();
        result.push('\n');
        for line in &self.body {
            result.push_str(line.text());
            result.push('\n');
        }
        result
    }

    /// provides a section with the given title
    #[cfg(test)]
    /// TODO: remove test builders like this and replace with scaffold / default
    pub fn with_title(title: &str) -> Section {
        Section {
            line_number: 0,
            title_line: Line::new(title),
            body: Vec::new(),
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

// -------------------------------------------------------------------------------------
// HELPERS
// -------------------------------------------------------------------------------------

/// allows building up sections one line at a time
pub struct Builder {
    pub line_number: u32,
    title_line: String,
    body: Vec<Line>,
}

impl Builder {
    /// Provides a builder instance loaded with the given title line.
    pub fn new<S: Into<String>>(title: S, line_number: u32) -> Builder {
        Builder {
            title_line: title.into(),
            line_number,
            body: Vec::new(),
        }
    }

    pub fn add_line<S: Into<String>>(&mut self, text: S) {
        self.body.push(Line::new(text));
    }

    /// Provides the content this builder has accumulated.
    pub fn result(self) -> Section {
        Section {
            title_line: Line::new(self.title_line),
            line_number: self.line_number,
            body: self.body,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::document::Document;
    use super::*;

    #[test]
    fn anchor() {
        let tests = vec![
            ("foo", "#foo"),
            ("what is it", "#what-is-it"),
            ("A Complex Section", "#a-complex-section"),
        ];
        for (give, want) in tests {
            let section = Section::with_title(give);
            assert_eq!(section.anchor(), want);
        }
    }

    mod last_line_abs {
        use crate::database::{Line, Section};

        #[test]
        fn no_body() {
            let section = Section {
                line_number: 12,
                ..Section::scaffold()
            };
            assert_eq!(section.last_line_abs(), 12);
        }

        #[test]
        fn with_body() {
            let section = Section {
                line_number: 12,
                body: vec![Line::new("")],
                ..Section::scaffold()
            };
            assert_eq!(section.last_line_abs(), 13);
        }
    }

    #[test]
    fn lines() {
        let give = "\
# test
title content";
        let doc = Document::from_str("foo", give).unwrap();
        let mut have = doc.title_section.lines();
        let line = have.next().expect("expected title line");
        assert_eq!(line.text(), "# test");
        let line = have.next().expect("expected body line 1");
        assert_eq!(line.text(), "title content");
        assert!(have.next().is_none(), "unexpected line");
    }

    mod push_line {
        use crate::database::{Line, Section};

        #[test]
        fn no_body() {
            let mut have = Section {
                body: vec![],
                ..Section::scaffold()
            };
            have.push_line("new line");
            let want = Section {
                body: vec![Line::new("new line")],
                ..Section::scaffold()
            };
            pretty::assert_eq!(have, want)
        }

        #[test]
        fn with_existing_body() {
            let mut section = Section {
                body: vec![Line::new("l1")],
                ..Section::scaffold()
            };
            section.push_line("new line");
            let have = section.body;
            let want = vec![Line::new("l1"), Line::new("new line")];
            pretty::assert_eq!(have, want)
        }
    }

    mod section_type {
        use crate::database::Section;

        #[test]
        fn h1() {
            let section = Section::with_title("# Title");
            assert_eq!(section.section_type(), "Title");
        }

        #[test]
        fn h3() {
            let section = Section::with_title("### Title");
            assert_eq!(section.section_type(), "Title");
        }

        #[test]
        fn no_header() {
            let section = Section::with_title("Title");
            assert_eq!(section.section_type(), "Title");
        }

        #[test]
        fn no_text() {
            let section = Section::with_title("###");
            assert_eq!(section.section_type(), "");
        }
    }

    #[test]
    fn text() {
        let section = Section {
            title_line: Line::new("### welcome"),
            body: vec![Line::new(""), Line::new("content")],
            ..Section::scaffold()
        };
        assert_eq!(section.text(), "### welcome\n\ncontent\n");
    }
}
