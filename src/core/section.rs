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

    pub fn section_type(&self) -> String {
        let pos = self
            .title_line
            .text
            .char_indices()
            .find(|(_, letter)| *letter != '#' && *letter != ' ');
        match pos {
            None => "".to_string(),
            Some((pos, _)) => self.title_line.text.clone().split_off(pos),
        }
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
    use std::path::PathBuf;

    #[test]
    fn anchor() {
        let tests = vec![
            ("foo", "#foo"),
            ("what is it", "#what-is-it"),
            ("A Complex Section", "#a-complex-section"),
        ];
        for (give, want) in tests.into_iter() {
            let section = Section {
                title_line: Line {
                    text: give.to_string(),
                    section_offset: 0,
                },
                body: vec![],
                line_number: 0,
            };
            assert_eq!(section.anchor(), want);
        }
    }

    #[test]
    fn lines() {
        let content = "\
# test
title content";
        let doc = Document::from_str(PathBuf::from("foo"), content);
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

    #[test]
    fn section_type() {
        let tests = vec![
            ("# Title", "Title"),
            ("### Title", "Title"),
            ("Title", "Title"),
            ("###", ""),
        ];
        for (give, want) in tests.into_iter() {
            let section = Section {
                line_number: 2,
                title_line: Line {
                    text: give.to_string(),
                    section_offset: 0,
                },
                body: vec![],
            };
            let have = section.section_type();
            assert_eq!(have, want.to_string(), "want: '{}', have: '{}'", want, have);
        }
    }

    #[test]
    fn text() {
        let section = Section {
            line_number: 12,
            title_line: Line {
                text: "### welcome".to_string(),
                section_offset: 0,
            },
            body: vec![
                Line {
                    section_offset: 0,
                    text: "".to_string(),
                },
                Line {
                    section_offset: 1,
                    text: "content".to_string(),
                },
            ],
        };
        assert_eq!(section.text(), "### welcome\n\ncontent\n");
    }
}
