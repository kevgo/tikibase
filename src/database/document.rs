use super::{section, Line, Section};
use crate::{Issue, Location};
use ahash::AHashSet;
use once_cell::sync::Lazy;
use regex::Regex;
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

static SOURCE_RE: Lazy<Regex> = Lazy::new(|| Regex::new("^(\\d+)\\.").unwrap());

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
                    if section.title().title == "occurrences" {
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
            if section.title().title == "occurrences" {
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

    #[cfg(test)]
    /// provides Document instances in tests
    pub fn from_str<P: Into<PathBuf>>(path: P, text: &str) -> Result<Document, Issue> {
        Document::from_lines(text.lines().map(|line| line.to_string()), path)
    }

    /// persists the changes made to this document to disk
    pub fn save(&self, root: &Path) {
        let mut file = fs::File::create(root.join(&self.path)).unwrap();
        file.write_all(self.text().as_bytes()).unwrap();
    }

    /// provides the section with the given title
    pub fn section_with_title(&self, title: &str) -> Option<&Section> {
        self.content_sections
            .iter()
            .find(|section| section.title().title == title)
    }

    pub fn last_line(&self) -> Option<&Line> {
        self.last_section().body.last()
    }

    pub fn last_section(&self) -> &Section {
        self.content_sections
            .last()
            .or(Some(&self.title_section))
            .unwrap()
    }

    /// provides the last section in this document
    pub fn last_section_mut(&mut self) -> &mut Section {
        self.content_sections
            .last_mut()
            .or(Some(&mut self.title_section))
            .unwrap()
    }

    /// provides the number of lines in this document
    pub fn lines_count(&self) -> u32 {
        self.content_sections
            .last()
            .or(Some(&self.title_section))
            .unwrap()
            .last_line_abs()
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
            .map(|section| section.title().title)
            .collect()
    }

    /// provides all the sources that this document defines
    pub fn sources_defined(&self) -> AHashSet<String> {
        let mut result = AHashSet::new();
        let links_section = match self.section_with_title("links") {
            None => return result,
            Some(section) => section,
        };
        for line in links_section.lines() {
            for cap in SOURCE_RE.captures_iter(line.text()) {
                result.insert(cap[1].to_string());
            }
        }
        result
    }

    /// provides all the sources used in this document
    pub fn sources_used(&self) -> AHashSet<UsedSource> {
        let mut result = AHashSet::new();
        let mut in_code_block = false;
        for section in self.sections() {
            if section.title().title == "occurrences" {
                continue;
            }
            for (line_idx, line) in section.lines().enumerate() {
                if line.text().starts_with("```") {
                    in_code_block = !in_code_block;
                    continue;
                }
                if !in_code_block {
                    for (index, start, end) in line.used_sources() {
                        result.insert(UsedSource {
                            line: section.line_number + (line_idx as u32),
                            index,
                            start,
                            end,
                        });
                    }
                }
            }
        }
        result
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
        self.title_section.title().title
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

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct UsedSource {
    /// the index used for the source, e.g. "[1]"
    pub index: String,
    pub line: u32,
    pub start: u32,
    pub end: u32,
}

// -------------------------------------------------------------------------------------
// TESTS
// -------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::Document;

    mod from_str {
        use super::super::Document;
        use crate::database::{Line, Section};
        use crate::{Issue, Location};
        use std::path::PathBuf;

        #[test]
        fn valid() {
            let give = "\
# test
### section 1
content";
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
            pretty::assert_eq!(have, want)
        }

        #[test]
        fn with_fenced_code_block() {
            let give = "\
# test
```md
### not a document section
text
```
";
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
            let give = "\
# test
```md
### not a document section
text
";
            let have = Document::from_str("test.md", give);
            let want = Err(Issue::UnclosedFence {
                location: Location {
                    file: PathBuf::from("test.md"),
                    line: 1,
                    start: 0,
                    end: 0,
                },
            });
            pretty::assert_eq!(have, want)
        }

        #[test]
        fn with_occurrences_section() {
            let give = "\
# test
### section 1
content
### occurrences
- occurrence 1
### links
- link 1";
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
        use super::super::Document;
        use crate::database::{Line, Section};

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
            let mut want = Section {
                line_number: 3,
                title_line: Line::from("### s1"),
                body: vec![Line::from(""), Line::from("text")],
            };
            pretty::assert_eq!(have, &mut want)
        }

        #[test]
        fn no_content_sections() {
            let give = "\
# Title
title text
";
            let mut doc = Document::from_str("test.md", give).unwrap();
            let have = doc.last_section_mut();
            let mut want = Section {
                line_number: 0,
                title_line: Line::from("# Title"),
                body: vec![Line::from("title text")],
            };
            pretty::assert_eq!(have, &mut want)
        }
    }

    #[test]
    fn section_titles() {
        let content = "\
# Title
title text
### Section 1
two
### Section 2
foo
";
        let doc = Document::from_str("test.md", content).unwrap();
        let have = doc.section_titles();
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

    mod sources_defined {
        use crate::database::document::Document;
        use ahash::AHashSet;

        #[test]
        fn no_links() {
            let give = "\
# Title
title text
";
            let doc = Document::from_str("test.md", give).unwrap();
            let have = doc.sources_defined();
            assert_eq!(have.len(), 0);
        }

        #[test]
        fn unordered_links() {
            let give = "\
# Title
title text
### links
- https://foo.com
";
            let doc = Document::from_str("test.md", give).unwrap();
            let have = doc.sources_defined();
            assert_eq!(have.len(), 0);
        }

        #[test]
        fn ordered_links() {
            let give = "\
# Title
title text
### links
1. https://one.com
2. https://two.com
";
            let doc = Document::from_str("test.md", give).unwrap();
            let have = doc.sources_defined();
            let mut want = AHashSet::new();
            want.insert("1".into());
            want.insert("2".into());
            assert_eq!(have, want);
        }
    }

    mod sources_used {
        use crate::database::document::{Document, UsedSource};
        use ahash::AHashSet;

        #[test]
        fn no_sources() {
            let give = "\
# Title
title text
";
            let doc = Document::from_str("test.md", give).unwrap();
            let have = doc.sources_used();
            assert_eq!(have.len(), 0);
        }

        #[test]
        fn with_sources() {
            let give = "\
# Title
title text [2]
### sec 1
text [1] [3]
";
            let doc = Document::from_str("test.md", give).unwrap();
            let have = doc.sources_used();
            let mut want = AHashSet::new();
            want.insert(UsedSource {
                line: 1,
                index: "2".into(),
                start: 11,
                end: 14,
            });
            want.insert(UsedSource {
                line: 3,
                index: "1".into(),
                start: 5,
                end: 8,
            });
            want.insert(UsedSource {
                line: 3,
                index: "3".into(),
                start: 9,
                end: 12,
            });
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn code_segment() {
            let give = "\
# Title
Example code: `map[0]`
";
            let doc = Document::from_str("test.md", give).unwrap();
            let have = doc.sources_used();
            assert_eq!(have.len(), 0);
        }

        #[test]
        fn code_block() {
            let give = "\
# Title
Example code:
```
map[0]
```
";
            let doc = Document::from_str("test.md", give).unwrap();
            let have = doc.sources_used();
            assert_eq!(have.len(), 0);
        }
    }
}
