use super::Line;
use heck::ToKebabCase;

/// a section in a document, from one heading to above the next heading
#[derive(Debug, PartialEq)]
pub struct Section {
    /// the line number at which this section starts, 0-based
    pub line_number: u32,

    /// complete textual content of this section's title line, e.g. "# Title"
    title_line: TitleLine,

    /// optional content of this section
    pub body: Vec<Line>,
}

/// the title line of a section
#[derive(Debug, PartialEq)]
struct TitleLine {
    pub line: Line,

    /// the cursor column at which the title text starts (derived value)
    // TODO: rename to text_start
    pub start: usize,

    /// cache for the heading level (h1-h6)
    pub level: u8,
}

impl TitleLine {
    pub fn end(&self) -> u32 {
        self.line.text.len() as u32
    }

    /// provides a human-readable description of this section, e.g. "Hello" for a section with the title "# Hello"
    pub fn text(&self) -> &str {
        &self.line.text[self.start..]
    }
}

impl Section {
    /// provides the link anchor for this section, in GitHub format
    pub fn anchor(&self) -> String {
        format!("#{}", self.title().to_kebab_case())
    }

    /// returns the last line of this section
    pub fn last_line(&self) -> &Line {
        match self.body.last() {
            Some(last_body_line) => last_body_line,
            None => &self.title_line.line,
        }
    }

    /// provides the absolute line number of the last line in this section
    pub fn last_line_abs(&self) -> u32 {
        self.line_number + (self.body.len() as u32)
    }

    /// provides the heading level (h1-h6) of this section
    pub fn level(&self) -> u8 {
        self.title_line.level
    }

    /// provides a non-consuming iterator for all lines in this section
    pub fn lines(&self) -> LinesIterator {
        LinesIterator {
            title_line: &self.title_line.line,
            body_iter: self.body.iter(),
            emitted_title: false,
        }
    }

    pub fn new<IS: Into<String>>(line_number: u32, title: IS, body: Vec<IS>) -> Section {
        let title: String = title.into();
        let mut chars = title.char_indices();
        let mut level = 0;
        while let Some((i, c)) = chars.next() {
            if c != '#' {
                level = i as u8;
                break;
            }
        }
        let mut start = 0;
        while let Some((i, c)) = chars.next() {
            if c != ' ' {
                start = i;
                break;
            }
        }
        if level == 0 {
            panic!("section has no level")
        }
        Section {
            line_number,
            title_line: TitleLine {
                line: Line::from(title),
                start,
                level,
            },
            body: body.into_iter().map(Line::from).collect(),
        }
    }

    /// adds a new line with the given text to this section
    pub fn push_line<S: Into<String>>(&mut self, text: S) {
        self.body.push(Line::from(text));
    }

    #[cfg(test)]
    fn scaffold() -> Self {
        Section::new(0, "### section", vec![])
    }

    /// provides the complete text of this section
    pub fn text(&self) -> String {
        let mut result = self.title_line.line.text.to_string();
        result.push('\n');
        for line in &self.body {
            result.push_str(&line.text);
            result.push('\n');
        }
        result
    }

    /// provides a human-readable description of this section, e.g. "Hello" for a section with the title "# Hello"
    pub fn title(&self) -> &str {
        &self.title_line.line.text[self.title_line.start..]
    }

    /// provides a section with the given title
    #[cfg(test)]
    pub fn with_title(title: &str) -> Section {
        Section::new(0, title, vec![])
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
            return Some(self.title_line);
        }
        self.body_iter.next()
    }
}

// -------------------------------------------------------------------------------------
// HELPERS
// -------------------------------------------------------------------------------------

/// allows building up sections one line at a time
pub struct Builder {
    pub line_number: u32,
    title_line: String,
    body: Vec<String>,
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

    pub fn add_line<IS: Into<String>>(&mut self, text: IS) {
        self.body.push(text.into());
    }

    /// Provides the content this builder has accumulated.
    pub fn result(self) -> Section {
        Section::new(self.line_number, self.title_line, self.body)
    }
}

#[cfg(test)]
mod tests {
    use super::super::document::Document;
    use super::*;
    use indoc::indoc;

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

    mod last_line {
        use crate::database::{Line, Section};

        #[test]
        fn with_body() {
            let section = Section {
                body: vec![Line::from("one"), Line::from("two")],
                ..Section::scaffold()
            };
            let have = section.last_line();
            let want = Line::from("two");
            assert_eq!(have, &want);
        }

        #[test]
        fn without_body() {
            let section = Section::new(0, "title", vec![]);
            let have = section.last_line();
            let want = Line::from("title");
            assert_eq!(have, &want);
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
                body: vec![Line::from("")],
                ..Section::scaffold()
            };
            assert_eq!(section.last_line_abs(), 13);
        }
    }

    #[test]
    fn level() {
        let tests = vec![("# title", 1), ("###### title", 6), ("###", 3)];
        for (give, want) in tests {
            let section = Section::with_title(give);
            assert_eq!(section.level(), want);
        }
    }

    #[test]
    fn lines() {
        let give = indoc! {"
            # test
            title content"};
        let doc = Document::from_str("foo", give).unwrap();
        let mut have = doc.title_section.lines();
        let line = have.next().expect("expected title line");
        assert_eq!(line.text, "# test");
        let line = have.next().expect("expected body line 1");
        assert_eq!(line.text, "title content");
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
                body: vec![Line::from("new line")],
                ..Section::scaffold()
            };
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn with_existing_body() {
            let mut section = Section {
                body: vec![Line::from("l1")],
                ..Section::scaffold()
            };
            section.push_line("new line");
            let have = section.body;
            let want = vec![Line::from("l1"), Line::from("new line")];
            pretty::assert_eq!(have, want);
        }
    }

    #[test]
    fn title() {
        let tests = vec![("# title", "title"), ("###### title", "title"), ("###", "")];
        for (give, want) in tests {
            let section = Section::with_title(give);
            assert_eq!(section.title(), want);
        }
    }

    #[test]
    fn text() {
        let section = Section::new(
            0,
            "### welcome",
            vec![Line::from(""), Line::from("content")],
        );
        assert_eq!(section.text(), "### welcome\n\ncontent\n");
    }
}
