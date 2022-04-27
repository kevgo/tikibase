use super::{Line, Reference};
use crate::{Config, Issue, Location};
use heck::ToKebabCase;
use std::path::Path;

/// a section in a document, from one heading to above the next heading
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Section {
    /// the line number at which this section starts, 0-based
    pub line_number: u32,

    /// complete textual content of this section's title line, e.g. "# Title"
    pub title_line: Line,

    /// optional content of this section
    pub body: Vec<Line>,

    /// the cursor column at which the title text starts (derived value),
    /// counterpart to `end`
    pub title_text_start: usize,

    /// cache for the heading level (h1-h6)
    pub level: u8,
}

impl Section {
    /// provides the link anchor for this section, in GitHub format
    pub fn anchor(&self) -> String {
        format!("#{}", self.human_title().to_kebab_case())
    }

    /// populates the given issues list if this section has no content
    pub fn check_empty(&self, path: &Path, issues: &mut Vec<Issue>) {
        if !self.body.iter().any(|line| !line.text.is_empty()) {
            issues.push(Issue::EmptySection {
                location: Location {
                    file: path.into(),
                    line: self.line_number,
                    start: 0,
                    end: self.title_line.text.len() as u32,
                },
                title: self.human_title().into(),
            });
        }
    }

    /// populates the given issues list if this section has an empty title
    pub fn check_empty_title(&self, path: &Path, issues: &mut Vec<Issue>) {
        if self.human_title().is_empty() {
            issues.push(Issue::SectionWithoutHeader {
                location: Location {
                    file: path.into(),
                    line: self.line_number,
                    start: 0,
                    end: self.title_text_end(),
                },
            });
        }
    }

    /// populates the given issues list with all sections in this document that don't match the configured sections
    pub fn check_mismatching_title(&self, path: &Path, config: &Config, issues: &mut Vec<Issue>) {
        let section_title = self.human_title();
        if !config.matching_title(section_title) {
            issues.push(Issue::UnknownSection {
                location: Location {
                    file: path.into(),
                    line: self.line_number,
                    start: self.title_text_start as u32,
                    end: self.title_text_end(),
                },
                title: section_title.into(),
                allowed_titles: config.sections.clone().unwrap(),
            });
        }
    }

    /// provides the cursor column at which the title text ends,
    /// counterpart to `start`
    pub fn title_text_end(&self) -> u32 {
        self.title_line.text.len() as u32
    }

    /// provides a human-readable version of this section's title, e.g. "Hello" for a section with the title "# Hello"
    pub fn human_title(&self) -> &str {
        &self.title_line.text[self.title_text_start..]
    }

    /// returns the last line of this section
    pub fn last_line(&self) -> &Line {
        match self.body.last() {
            Some(last_body_line) => last_body_line,
            None => &self.title_line,
        }
    }

    /// provides the absolute line number of the last line in this section
    pub fn last_line_abs(&self) -> u32 {
        self.line_number + (self.body.len() as u32)
    }

    /// provides a non-consuming iterator for all lines in this section
    pub fn lines(&self) -> LinesIterator {
        LinesIterator {
            title_line: &self.title_line,
            body_iter: self.body.iter(),
            emitted_title: false,
        }
    }

    pub fn new<IS: Into<String>>(line_number: u32, title: IS, body: Vec<IS>) -> Section {
        let title: String = title.into();
        let mut chars = title.char_indices();
        let mut level = 0;
        for (i, c) in chars.by_ref() {
            if c != '#' {
                level = i;
                break;
            }
        }
        if level == 0 {
            level = title.len();
        }
        let mut start = level;
        for (i, c) in chars {
            if c != ' ' {
                start = i;
                break;
            }
        }
        Section {
            line_number,
            title_line: Line::from(title),
            title_text_start: start,
            level: level as u8,
            body: body.into_iter().map(Line::from).collect(),
        }
    }

    /// adds a new line with the given text to this section
    pub fn push_line<IS: Into<String>>(&mut self, text: IS) {
        self.body.push(Line::from(text));
    }

    pub fn references(&self, acc: &mut Vec<Reference>) {
        self.title_line.references(self.line_number, acc);
        for (i, line) in self.body.iter().enumerate() {
            line.references(self.line_number + i as u32 + 1, acc);
        }
    }

    #[cfg(test)]
    fn scaffold() -> Self {
        Section::new(0, "### section", vec![])
    }

    /// provides the complete text of this section
    pub fn text(&self) -> String {
        let mut result = self.title_line.text.to_string();
        result.push('\n');
        for line in &self.body {
            result.push_str(&line.text);
            result.push('\n');
        }
        result
    }

    /// provides a section with the given title
    #[cfg(test)]
    pub fn with_body(body: Vec<&str>) -> Section {
        Section::new(0, "# title", body)
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

/// allows building up sections one line at a time
// TODO: delete this?
pub struct Builder {
    pub line_number: u32,
    title_line: String,
    body: Vec<String>,
}

impl Builder {
    /// Provides a builder instance loaded with the given title line.
    pub fn new<IS: Into<String>>(title: IS, line_number: u32) -> Builder {
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
            ("### foo", "#foo"),
            ("### A Complex Section", "#a-complex-section"),
        ];
        for (give, want) in tests {
            let section = Section::with_title(give);
            assert_eq!(section.anchor(), want);
        }
    }
    mod check_empty {
        use crate::database::Document;
        use crate::{Issue, Location};
        use indoc::indoc;
        use std::path::PathBuf;

        #[test]
        fn empty_section() {
            let content = indoc! {"
            # test document

            ### empty section
            ### next section

            content"};
            let doc = Document::from_str("test.md", content).unwrap();
            let mut have = vec![];
            for section in doc.content_sections {
                section.check_empty(&PathBuf::from("test.md"), &mut have);
            }
            let want = vec![Issue::EmptySection {
                location: Location {
                    file: PathBuf::from("test.md"),
                    line: 2,
                    start: 0,
                    end: 17,
                },
                title: "empty section".into(),
            }];
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn blank_line() {
            let content = indoc! {"
            # test document

            ### empty section

            ### next section

            content"};
            let doc = Document::from_str("test.md", content).unwrap();
            let mut have = vec![];
            for section in doc.content_sections {
                section.check_empty(&PathBuf::from("test.md"), &mut have);
            }
            let want = vec![Issue::EmptySection {
                location: Location {
                    file: PathBuf::from("test.md"),
                    line: 2,
                    start: 0,
                    end: 17,
                },
                title: "empty section".into(),
            }];
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn content() {
            let content = indoc! {"
            # test document

            ### section with content

            content"};
            let doc = Document::from_str("test.md", content).unwrap();
            let mut have = vec![];
            for section in doc.content_sections {
                section.check_empty(&PathBuf::from("test.md"), &mut have);
            }
            assert!(have.is_empty());
        }
    }

    mod check_empty_title {
        use crate::database::Document;
        use crate::{Issue, Location};
        use indoc::indoc;
        use std::path::PathBuf;

        #[test]
        fn empty_title() {
            let content = indoc! {"
            # test document

            ###
            content
            ###
            content"};
            let doc = Document::from_str("test.md", content).unwrap();
            let mut have = vec![];
            for section in doc.sections() {
                section.check_empty_title(&PathBuf::from("test.md"), &mut have);
            }
            let want = vec![
                Issue::SectionWithoutHeader {
                    location: Location {
                        file: PathBuf::from("test.md"),
                        line: 2,
                        start: 0,
                        end: 3,
                    },
                },
                Issue::SectionWithoutHeader {
                    location: Location {
                        file: PathBuf::from("test.md"),
                        line: 4,
                        start: 0,
                        end: 3,
                    },
                },
            ];
            pretty::assert_eq!(have, want);
        }
    }

    #[test]
    fn human_title() {
        let tests = vec![("# title", "title"), ("###### title", "title"), ("###", "")];
        for (give, want) in tests {
            let section = Section::with_title(give);
            assert_eq!(section.human_title(), want);
        }
    }

    mod last_line {
        use crate::database::{Line, Section};

        #[test]
        fn has_body() {
            let section = Section::with_body(vec!["one", "two"]);
            let have = section.last_line();
            let want = Line::from("two");
            assert_eq!(have, &want);
        }

        #[test]
        fn no_body() {
            let section = Section::new(0, "### title", vec![]);
            let have = section.last_line();
            let want = Line::from("### title");
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
            assert_eq!(section.level, want);
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
    fn text() {
        let section = Section::new(0, "### welcome", vec!["", "content"]);
        assert_eq!(section.text(), "### welcome\n\ncontent\n");
    }
}
