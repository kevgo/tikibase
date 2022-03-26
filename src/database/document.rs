use super::{section, Footnotes, Line, Reference, Section};
use crate::{Issue, Location};
use std::fs;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq)]
pub struct Document {
    /// the path relative to the Tikibase root directory
    pub path: PathBuf,
    pub title_section: Section,
    pub content_sections: Vec<Section>,
    /// The old "occurrences" section that was filtered out when loading the document.
    pub old_occurrences_section: Option<Section>,
}

impl Document {
    /// provides a Document instance containing the given text
    pub fn from_lines<T, P: Into<PathBuf>>(lines: T, path: P) -> Result<Document, Issue>
    where
        T: Iterator<Item = String>,
    {
        let path = path.into();
        let mut sections: Vec<Section> = Vec::new();
        let mut section_builder: Option<section::Builder> = None;
        let mut inside_fence = false;
        let mut fence_line = 0;
        let mut old_occurrences_section: Option<Section> = None;
        for (line_number, line) in lines.enumerate() {
            if line.starts_with('#') && !inside_fence {
                if let Some(section_builder) = section_builder {
                    let section = section_builder.result();
                    if section.title().text == "occurrences" {
                        old_occurrences_section = Some(section);
                    } else {
                        sections.push(section);
                    }
                }
                section_builder = Some(section::Builder::new(line, line_number as u32));
                continue;
            }
            if line.starts_with("```") {
                inside_fence = !inside_fence;
                fence_line = line_number;
            }
            match &mut section_builder {
                Some(section_builder) => section_builder.add_line(line),
                None => {
                    return Err(Issue::NoTitleSection {
                        location: Location {
                            file: path,
                            line: line_number as u32,
                            start: 0,
                            end: line.len() as u32,
                        },
                    })
                }
            }
        }
        if let Some(section_builder) = section_builder {
            let section = section_builder.result();
            if section.title().text == "occurrences" {
                old_occurrences_section = Some(section);
            } else {
                sections.push(section);
            }
        }
        if inside_fence {
            return Err(Issue::UnclosedFence {
                location: Location {
                    file: path,
                    line: (fence_line as u32),
                    start: 0,
                    end: 0,
                },
            });
        }
        let mut sections = sections.into_iter();
        Ok(Document {
            path,
            title_section: sections.next().unwrap(),
            content_sections: sections.collect(),
            old_occurrences_section,
        })
    }

    /// provides all the footnotes that this document defines and references
    pub fn footnotes(&self) -> Result<Footnotes, Issue> {
        let mut result = Footnotes::default();
        let mut code_block_start: Option<CodeblockStart> = None;
        for (i, line) in self.lines().enumerate() {
            if line.is_code_block_boundary() {
                code_block_start = match code_block_start {
                    Some(_) => None,
                    None => Some(CodeblockStart {
                        line: i as u32,
                        len: line.text.len() as u32,
                    }),
                };
                continue;
            }
            if code_block_start.is_none() {
                line.add_footnotes_to(&mut result, &self.path, i as u32)?;
            }
        }
        if let Some(code_block_start) = code_block_start {
            return Err(Issue::UnclosedFence {
                location: Location {
                    file: self.path.clone(),
                    line: code_block_start.line,
                    start: 0,
                    end: code_block_start.len,
                },
            });
        }
        Ok(result)
    }

    #[cfg(test)]
    /// provides Document instances in tests
    pub fn from_str<P: Into<PathBuf>>(path: P, text: &str) -> Result<Document, Issue> {
        Document::from_lines(text.lines().map(std::string::ToString::to_string), path)
    }

    /// provides the last line in this document
    pub fn last_line(&self) -> &Line {
        self.last_section().last_line()
    }

    /// provides the last section in this document
    pub fn last_section(&self) -> &Section {
        match self.content_sections.last() {
            Some(last_content_section) => last_content_section,
            None => &self.title_section,
        }
    }

    /// provides the last section in this document
    pub fn last_section_mut(&mut self) -> &mut Section {
        self.content_sections
            .last_mut()
            .or(Some(&mut self.title_section))
            .unwrap()
    }

    /// provides an iterator over all lines in this document
    pub fn lines(&self) -> LinesIterator {
        let mut section_iter = self.sections();
        let section = section_iter.next().unwrap();
        LinesIterator {
            section_iter,
            lines_iter: section.lines(),
        }
    }

    /// provides the number of lines in this document
    pub fn lines_count(&self) -> u32 {
        self.content_sections
            .last()
            .or(Some(&self.title_section))
            .unwrap()
            .last_line_abs()
    }

    /// provides the Document contained in the file with the given path
    pub fn from_reader<R: BufRead, P: Into<PathBuf>>(
        reader: R,
        path: P,
    ) -> Result<Document, Issue> {
        let lines = reader.lines().map(Result::unwrap);
        Document::from_lines(lines, path)
    }

    /// provides all the references in this document
    pub fn references(&self) -> Vec<Reference> {
        let mut result = vec![];
        for (i, line) in self.lines().enumerate() {
            result.append(&mut line.references(i as u32));
        }
        result
    }

    /// persists the changes made to this document to disk
    pub fn save(&self, root: &Path) {
        let mut file = fs::File::create(root.join(&self.path)).unwrap();
        file.write_all(self.text().as_bytes()).unwrap();
    }

    /// provides a non-consuming iterator for all sections in this document
    pub fn sections(&self) -> SectionIterator {
        SectionIterator {
            title_section: &self.title_section,
            body_iter: self.content_sections.iter(),
            emitted_title: false,
        }
    }

    /// provides the section titles in this document
    pub fn section_titles(&self) -> Vec<&str> {
        self.content_sections
            .iter()
            .map(|section| section.title().text)
            .collect()
    }

    /// provides the section with the given title
    pub fn section_with_title(&self, title: &str) -> Option<&Section> {
        self.content_sections
            .iter()
            .find(|section| section.title().text == title)
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
    pub fn title(&self) -> &str {
        self.title_section.title().text
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
            return Some(self.title_section);
        }
        self.body_iter.next()
    }
}

/// iterates over all lines in a Document
pub struct LinesIterator<'a> {
    /// to get the next section
    section_iter: SectionIterator<'a>,
    /// iterator over the lines in the current section
    lines_iter: section::LinesIterator<'a>,
}

impl<'a> Iterator for LinesIterator<'a> {
    type Item = &'a Line;

    fn next(&mut self) -> Option<Self::Item> {
        let next_line = self.lines_iter.next();
        if next_line.is_some() {
            return next_line;
        }
        let next_section = match self.section_iter.next() {
            Some(section) => section,
            None => return None,
        };
        self.lines_iter = next_section.lines();
        self.lines_iter.next()
    }
}

/// describes the start of a codeblock
struct CodeblockStart {
    /// the line on which this codeblock starts
    line: u32,
    /// length of the text on this line
    len: u32,
}

// -------------------------------------------------------------------------------------
// TESTS
// -------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::Document;
    use crate::database::Reference;
    use indoc::indoc;

    mod footnotes {
        use crate::database::{Document, Footnote, Footnotes};
        use indoc::indoc;

        #[test]
        fn no_footnotes() {
            let give = indoc! {"
                # Title
                title text
                "};
            let have = Document::from_str("test.md", give).unwrap().footnotes();
            let want = Ok(Footnotes::default());
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn has_footnotes() {
            let give = indoc! {"
                # Title
                reference to [^1]
                100 tons of [^rust]
                ### links
                [^1]: first footnote
                [^second]: second footnote
                "};
            let have = Document::from_str("test.md", give).unwrap().footnotes();
            let want = Ok(Footnotes {
                definitions: vec![
                    Footnote {
                        identifier: "1".into(),
                        line: 4,
                        start: 0,
                        end: 5,
                    },
                    Footnote {
                        identifier: "second".into(),
                        line: 5,
                        start: 0,
                        end: 10,
                    },
                ],
                references: vec![
                    Footnote {
                        identifier: "1".into(),
                        line: 1,
                        start: 13,
                        end: 17,
                    },
                    Footnote {
                        identifier: "rust".into(),
                        line: 2,
                        start: 12,
                        end: 19,
                    },
                ],
            });
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn code_block() {
            let give = indoc! {"
                # Title
                ```
                [^1]
                ```
                "};
            let have = Document::from_str("test.md", give).unwrap().footnotes();
            let want = Ok(Footnotes::default());
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn code_segment() {
            let give = indoc! {"
                # Title
                a `[^1]` code block
                "};
            let have = Document::from_str("test.md", give).unwrap().footnotes();
            let want = Ok(Footnotes::default());
            pretty::assert_eq!(have, want);
        }
    }

    mod from_str {
        use super::super::Document;
        use crate::database::{Line, Section};
        use crate::{Issue, Location};
        use indoc::indoc;
        use std::path::PathBuf;

        #[test]
        fn valid() {
            let give = indoc! {"
                # test
                ### section 1
                content"};
            let have = Document::from_str("one.md", give);
            let want = Ok(Document {
                path: PathBuf::from("one.md"),
                title_section: Section {
                    line_number: 0,
                    title_line: Line::from("# test"),
                    body: vec![],
                },
                content_sections: vec![Section {
                    line_number: 1,
                    title_line: Line::from("### section 1"),
                    body: vec![Line::from("content")],
                }],
                old_occurrences_section: None,
            });
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn missing_title() {
            let have = Document::from_str("one.md", "no title");
            let want = Err(Issue::NoTitleSection {
                location: Location {
                    file: PathBuf::from("one.md"),
                    line: 0,
                    start: 0,
                    end: 8,
                },
            });
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn with_fenced_code_block() {
            let give = indoc! {"
                # test
                ```md
                ### not a document section
                text
                ```
                "};
            let have = Document::from_str("test.md", give);
            let want = Ok(Document {
                path: PathBuf::from("test.md"),
                title_section: Section {
                    line_number: 0,
                    title_line: Line::from("# test"),
                    body: vec![
                        Line::from("```md"),
                        Line::from("### not a document section"),
                        Line::from("text"),
                        Line::from("```"),
                    ],
                },
                content_sections: vec![],
                old_occurrences_section: None,
            });
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn open_fenced_code_block() {
            let give = indoc! {"
                # test
                ```md
                ### not a document section
                text
                "};
            let have = Document::from_str("test.md", give);
            let want = Err(Issue::UnclosedFence {
                location: Location {
                    file: PathBuf::from("test.md"),
                    line: 1,
                    start: 0,
                    end: 0,
                },
            });
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn with_occurrences_section() {
            let give = indoc! {"
                # test
                ### section 1
                content
                ### occurrences
                - occurrence 1
                ### links
                - link 1"};
            let have = Document::from_str("one.md", give);
            let want = Ok(Document {
                path: PathBuf::from("one.md"),
                title_section: Section {
                    line_number: 0,
                    title_line: Line::from("# test"),
                    body: vec![],
                },
                content_sections: vec![
                    Section {
                        line_number: 1,
                        title_line: Line::from("### section 1"),
                        body: vec![Line::from("content")],
                    },
                    Section {
                        line_number: 5,
                        title_line: Line::from("### links"),
                        body: vec![Line::from("- link 1")],
                    },
                ],
                old_occurrences_section: Some(Section {
                    line_number: 3,
                    title_line: Line::from("### occurrences"),
                    body: vec![Line::from("- occurrence 1")],
                }),
            });
            pretty::assert_eq!(have, want);
        }
    }

    mod last_line {
        use crate::database::{Document, Line};

        #[test]
        fn title_section_only() {
            let doc = Document::from_str("test.md", "# Title\ntitle text\n").unwrap();
            let have = doc.last_line();
            let want = Line::from("title text");
            pretty::assert_eq!(have, &want);
        }

        #[test]
        fn with_body() {
            let doc =
                Document::from_str("test.md", "# Title\n### section 1\nsection text").unwrap();
            let have = doc.last_line();
            let want = Line::from("section text");
            pretty::assert_eq!(have, &want);
        }

        #[test]
        fn title_only() {
            let doc = Document::from_str("test.md", "# Title").unwrap();
            let have = doc.last_line();
            let want = Line::from("# Title");
            pretty::assert_eq!(have, &want);
        }

        #[test]
        fn section_without_body() {
            let doc = Document::from_str("test.md", "# Title\n### section 1").unwrap();
            let have = doc.last_line();
            let want = Line::from("### section 1");
            pretty::assert_eq!(have, &want);
        }
    }

    mod last_section {
        use crate::database::Document;

        #[test]
        fn title_only() {
            let text = &"# Title";
            let doc = Document::from_str("test.md", text).unwrap();
            let have = doc.last_section();
            let want = &doc.title_section;
            pretty::assert_eq!(&have, &want);
        }

        #[test]
        fn with_body() {
            let text = &"# Title\n### section 1\nsection text";
            let doc = Document::from_str("test.md", text).unwrap();
            let have = doc.last_section();
            let want = &doc.content_sections[0];
            pretty::assert_eq!(&have, &want);
        }
    }

    mod last_section_mut {
        use super::super::Document;
        use crate::database::{Line, Section};
        use indoc::indoc;

        #[test]
        fn has_content_section() {
            let give = indoc! {"
                # Title
                title text

                ### s1

                text
                "};
            let mut doc = Document::from_str("test.md", give).unwrap();
            let have = doc.last_section_mut();
            let mut want = Section {
                line_number: 3,
                title_line: Line::from("### s1"),
                body: vec![Line::from(""), Line::from("text")],
            };
            pretty::assert_eq!(have, &mut want);
        }

        #[test]
        fn no_content_sections() {
            let give = indoc! {"
                # Title
                title text
                "};
            let mut doc = Document::from_str("test.md", give).unwrap();
            let have = doc.last_section_mut();
            let mut want = Section {
                line_number: 0,
                title_line: Line::from("# Title"),
                body: vec![Line::from("title text")],
            };
            pretty::assert_eq!(have, &mut want);
        }
    }

    mod lines {
        use super::super::Document;
        use crate::database::Line;
        use indoc::indoc;

        #[test]
        fn multiple_sections() {
            let give = indoc! {"
            # Title
            title text

            ### Section 1
            one
            two

            ### Section 2
            foo
            "};
            let doc = Document::from_str("test.md", give).unwrap();
            let mut lines = doc.lines();
            pretty::assert_eq!(lines.next(), Some(&Line::from("# Title")));
            pretty::assert_eq!(lines.next(), Some(&Line::from("title text")));
            pretty::assert_eq!(lines.next(), Some(&Line::from("")));
            pretty::assert_eq!(lines.next(), Some(&Line::from("### Section 1")));
            pretty::assert_eq!(lines.next(), Some(&Line::from("one")));
            pretty::assert_eq!(lines.next(), Some(&Line::from("two")));
            pretty::assert_eq!(lines.next(), Some(&Line::from("")));
            pretty::assert_eq!(lines.next(), Some(&Line::from("### Section 2")));
            pretty::assert_eq!(lines.next(), Some(&Line::from("foo")));
            pretty::assert_eq!(lines.next(), None);
        }

        #[test]
        fn section_without_body() {
            let give = indoc! {"
                # Title
                ### Section 1
                ### Section 2
                "};
            let doc = Document::from_str("test.md", give).unwrap();
            let mut lines = doc.lines();
            pretty::assert_eq!(lines.next(), Some(&Line::from("# Title")));
            pretty::assert_eq!(lines.next(), Some(&Line::from("### Section 1")));
            pretty::assert_eq!(lines.next(), Some(&Line::from("### Section 2")));
            pretty::assert_eq!(lines.next(), None);
        }
    }

    mod lines_count {
        use super::super::Document;
        use indoc::indoc;

        #[test]
        fn with_content_sections() {
            let give = indoc! {"
                # Title
                title text
                ### Section 1
                one
                two
                ### Section 2
                foo
                "};
            let have = Document::from_str("test.md", give).unwrap().lines_count();
            assert_eq!(have, 6);
        }

        #[test]
        fn no_content_sections() {
            let give = indoc! {"
                # Title
                title text
                "};
            let have = Document::from_str("test.md", give).unwrap().lines_count();
            assert_eq!(have, 1);
        }
    }

    #[test]
    fn references() {
        let give = indoc! {"
            # Title
            a link: [one](1.md)
            ### section
            an image: ![two](2.png)
            "};
        let have = Document::from_str("test.md", give).unwrap().references();
        let want = vec![
            Reference::Link {
                target: "1.md".into(),
                line: 1,
                start: 8,
                end: 19,
            },
            Reference::Image {
                src: "2.png".into(),
                line: 3,
                start: 10,
                end: 23,
            },
        ];
        pretty::assert_eq!(have, want);
    }

    #[test]
    fn section_titles() {
        let content = indoc! {"
            # Title
            title text
            ### Section 1
            two
            ### Section 2
            foo
            "};
        let doc = Document::from_str("test.md", content).unwrap();
        let have = doc.section_titles();
        let want = vec!["Section 1".to_string(), "Section 2".to_string()];
        assert_eq!(have, want);
    }

    #[test]
    fn text() {
        let give = indoc! {"
            # Title
            title text
            ### Section 1
            one
            two
            ### Section 2
            foo
            "};
        let have = Document::from_str("test.md", give).unwrap().text();
        assert_eq!(have, give);
    }

    #[test]
    fn title() {
        let give = indoc! {"
            # Title
            title text
            ### Section 1
            one
            "};
        let doc = Document::from_str("test.md", give).unwrap();
        let have = doc.title();
        assert_eq!(have, "Title");
    }
}
