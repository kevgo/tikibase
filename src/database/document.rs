use super::{section, Section};
use ahash::AHashSet;
use lazy_static::lazy_static;
use regex::Regex;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub struct Document {
    /// the path relative to the Tikibase root directory
    pub path: PathBuf,
    pub title_section: Section,
    pub content_sections: Vec<Section>,
    /// Loading filters out "occurrences" sections.
    /// Some => this document had an "occurrences" section at the given line when loading it.
    /// None => this document had no occurrences section when loading it.
    pub occurrences_section_line: Option<u32>,
}

impl Document {
    /// provides a Document instance containing the given text
    pub fn from_lines<T, P: Into<PathBuf>>(lines: T, path: P) -> Result<Document, String>
    where
        T: Iterator<Item = String>,
    {
        let path = path.into();
        let mut sections: Vec<Section> = Vec::new();
        let mut section_builder: Option<section::Builder> = None;
        let mut inside_fence = false;
        let mut fence_line = 0;
        let mut occurrences_section_line: Option<u32> = None;
        for (line_number, line) in lines.enumerate() {
            if line.starts_with('#') && !inside_fence {
                if let Some(section_builder) = section_builder {
                    let section = section_builder.result();
                    if section.section_type() == "occurrences" {
                        occurrences_section_line = Some(section.line_number);
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
                Some(section_builder) => section_builder.add_body_line(line),
                None => return Err(format!("{}  no title section", path.to_string_lossy())),
            }
        }
        if let Some(section_builder) = section_builder {
            let section = section_builder.result();
            if section.section_type() == "occurrences" {
                occurrences_section_line = Some(section.line_number);
            } else {
                sections.push(section);
            }
        }
        if inside_fence {
            return Err(format!(
                "{}:{}  unclosed fence",
                path.to_string_lossy(),
                fence_line + 1,
            ));
        }
        let content_sections = sections.split_off(1);
        Ok(Document {
            path,
            title_section: sections.pop().unwrap(),
            content_sections,
            occurrences_section_line,
        })
    }

    #[cfg(test)]
    /// provides Document instances in tests
    pub fn from_str<P: Into<PathBuf>>(path: P, text: &str) -> Result<Document, String> {
        Document::from_lines(text.lines().map(|line| line.to_string()), path)
    }

    /// persists the changes made to this document to disk
    pub fn save(&self, root: &Path) {
        let mut file = std::fs::File::create(root.join(&self.path)).unwrap();
        file.write_all(self.text().as_bytes()).unwrap();
    }

    /// provides the section with the given title
    pub fn section_with_title(&self, section_type: &str) -> Option<&Section> {
        self.content_sections
            .iter()
            .find(|section| section.section_type() == section_type)
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

    /// provides the section types in this document
    pub fn section_types(&self) -> Vec<&str> {
        self.content_sections
            .iter()
            .map(Section::section_type)
            .collect()
    }

    /// provides all the sources that this document defines
    pub fn sources_defined(&self) -> AHashSet<String> {
        let mut result = AHashSet::new();
        let links_section = match self.section_with_title("links") {
            None => return result,
            Some(section) => section,
        };
        lazy_static! {
            static ref SOURCE_RE: Regex = Regex::new("^(\\d+)\\.").unwrap();
        }
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
            if section.section_type() == "occurrences" {
                continue;
            }
            for (line_idx, line) in section.lines().enumerate() {
                if line.text().starts_with("```") {
                    in_code_block = !in_code_block;
                    continue;
                }
                if !in_code_block {
                    for index in line.used_sources() {
                        result.insert(UsedSource {
                            line: section.line_number + (line_idx as u32),
                            index,
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

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct UsedSource {
    pub line: u32,
    /// the index used for the source, e.g. "[1]"
    pub index: String,
}

// -------------------------------------------------------------------------------------
// TESTS
// -------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::Document;

    mod from_str {
        use super::super::Document;

        #[test]
        fn valid() {
            let content = "\
# test
### section 1
content";
            let doc = Document::from_str("one.md", content).unwrap();
            let mut sections = doc.sections();
            let section = sections.next().expect("expected title section");
            assert_eq!(section.title_line.text(), "# test");
            let section = sections.next().expect("expected s1");
            assert_eq!(section.title_line.text(), "### section 1");
            if sections.next().is_some() {
                panic!("unexpected section");
            }
        }

        #[test]
        fn invalid() {
            match Document::from_str("one.md", "content") {
                Err(e) => assert_eq!(e, "one.md  no title section"),
                Ok(_) => panic!(),
            }
        }

        #[test]
        fn with_fenced_code_block() {
            let content = "\
# test

```md
### not a document section
text
```
";
            let doc = Document::from_str("test.md", content).unwrap();
            assert_eq!(doc.content_sections.len(), 0);
            assert_eq!(doc.title_section.lines().count(), 6);
        }

        #[test]
        fn with_open_fenced_code_block() {
            let content = "\
# test

```md
### not a document section
text
";
            match Document::from_str("test.md", content) {
                Err(msg) => assert_eq!(msg, "test.md:3  unclosed fence"),
                Ok(_) => panic!(),
            }
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
            assert_eq!(have.title_line.text(), "### s1");
        }

        #[test]
        fn no_content_sections() {
            let give = "\
# Title
title text
";
            let mut doc = Document::from_str("test.md", give).unwrap();
            let have = doc.last_section_mut();
            assert_eq!(have.title_line.text(), "# Title");
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
            });
            want.insert(UsedSource {
                line: 3,
                index: "1".into(),
            });
            want.insert(UsedSource {
                line: 3,
                index: "3".into(),
            });
            assert_eq!(have, want);
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
