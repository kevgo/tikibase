use super::{section, Directory, EntryType, Footnotes, Line, Reference, Section};
use crate::{Config, Issue, Location};
use ahash::AHashMap;
use std::ffi::OsString;
use std::fs::{self, File};
use std::io::{prelude::*, BufReader};
use std::path::{Path, PathBuf};

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Document {
    // TODO: make these elements private once the move to new architecture is complete
    /// the path relative to the Tikibase root directory
    pub relative_path: PathBuf,
    pub title_section: Section,
    pub content_sections: Vec<Section>,
    /// The old "occurrences" section that was filtered out when loading the document.
    pub old_occurrences_section: Option<Section>,

    /// cache of files this document links to
    // TODO: convert to HashSet and use https://github.com/mcarton/rust-derivative to ignore this when hashing Document
    pub references: Vec<Reference>,
}

impl Document {
    // populates the given issues list with all issues in this document
    pub fn check(
        &self,
        path: &Path,
        dir: &Path,
        config: &Config,
        issues: &mut Vec<Issue>,
        linked_resources: &mut Vec<PathBuf>,
        root: &Directory,
    ) {
        self.find_duplicate_sections(path, issues);
        self.find_unordered_sections(path, config, issues);
        self.find_mismatching_footnotes(path, issues);
        self.check_links(path, dir, issues, linked_resources, root, config);
        self.title_section.check_empty_title(path, issues);
        for section in &self.content_sections {
            section.check_empty(path, issues);
            section.check_empty_title(path, issues);
            section.check_mismatching_title(path, config, issues);
        }
    }

    /// populates the given issues list with all link issues in this document
    pub fn check_links(
        &self,
        path: &Path,
        dir: &Path,
        issues: &mut Vec<Issue>,
        linked_resources: &mut Vec<PathBuf>,
        root: &Directory,
        config: &Config,
    ) {
        if self.references.is_empty() {
            issues.push(Issue::DocumentWithoutLinks {
                location: Location {
                    file: path.into(),
                    line: 0,
                    start: 0,
                    end: 0,
                },
            });
        }
        for reference in &self.references {
            match reference {
                Reference::Link {
                    target,
                    line,
                    start,
                    end,
                } => {
                    if target.is_empty() {
                        issues.push(Issue::LinkWithoutTarget {
                            location: Location {
                                file: path.into(),
                                line: line.to_owned(),
                                start: start.to_owned(),
                                end: end.to_owned(),
                            },
                        });
                        continue;
                    }
                    if target.starts_with("http") {
                        // ignore external links
                        continue;
                    }
                    let (target_file, target_anchor) = match target.split_once('#') {
                        Some((base, anchor)) => (base.to_string(), format!("#{}", anchor)),
                        None => (target.clone(), "".to_string()),
                    };
                    let path_str = path.to_string_lossy();
                    if target_file == path_str {
                        issues.push(Issue::LinkToSameDocument {
                            location: Location {
                                file: path.into(),
                                line: line.to_owned(),
                                start: start.to_owned(),
                                end: end.to_owned(),
                            },
                        });
                        continue;
                    }
                    if target.starts_with('#')
                        && !self
                            .content_sections
                            .iter()
                            .any(|section| &section.anchor() == target)
                    {
                        issues.push(Issue::LinkToNonExistingAnchorInCurrentDocument {
                            location: Location {
                                file: path.into(),
                                line: line.to_owned(),
                                start: start.to_owned(),
                                end: end.to_owned(),
                            },
                            anchor: target.clone(),
                        });
                        continue;
                    }
                    match EntryType::from_str(&target_file) {
                        EntryType::Document => {
                            if let Some(doc) = root.get_doc(&target_file) {
                                if !target_anchor.is_empty() && !doc.has_anchor(&target_anchor) {
                                    issues.push(Issue::LinkToNonExistingAnchorInExistingDocument {
                                        location: Location {
                                            file: path.into(),
                                            line: line.to_owned(),
                                            start: start.to_owned(),
                                            end: end.to_owned(),
                                        },
                                        target_file: target_file.clone(),
                                        anchor: target_anchor,
                                    });
                                    // continue;
                                }
                                // check for backlink from doc to us
                                if let Some(bidi_links) = config.bidi_links {
                                    if bidi_links && !doc.contains_reference_to(path) {
                                        issues.push(Issue::MissingLink {
                                            location: Location {
                                                file: PathBuf::from(target_file),
                                                line: doc.lines_count(),
                                                start: 0,
                                                end: 0,
                                            },
                                            path: path.into(),
                                            title: self.human_title().into(),
                                        });
                                    }
                                }
                            } else {
                                issues.push(Issue::LinkToNonExistingFile {
                                    location: Location {
                                        file: path.into(),
                                        line: line.to_owned(),
                                        start: start.to_owned(),
                                        end: end.to_owned(),
                                    },
                                    target: target.into(),
                                });
                                continue;
                            };
                        }
                        EntryType::Resource => {
                            if !root.has_resource(&target_file) {
                                issues.push(Issue::LinkToNonExistingFile {
                                    location: Location {
                                        file: path.into(),
                                        line: line.to_owned(),
                                        start: start.to_owned(),
                                        end: end.to_owned(),
                                    },
                                    target: target.into(),
                                });
                                continue;
                            }
                            linked_resources.push(dir.join(&target_file));
                        }
                        EntryType::Configuration | EntryType::Ignored => {}
                        EntryType::Directory => todo!(),
                    }
                }
                Reference::Image {
                    src,
                    line,
                    start,
                    end,
                } => {
                    if src.starts_with("http") {
                        continue;
                    }
                    if !root.has_resource(&src) {
                        issues.push(Issue::BrokenImage {
                            location: Location {
                                file: path.into(),
                                line: line.to_owned(),
                                start: start.to_owned(),
                                end: end.to_owned(),
                            },
                            target: src.clone(),
                        });
                        continue;
                    }
                    linked_resources.push(dir.join(src));
                }
            }
        }
    }

    pub fn contains_reference_to<P: AsRef<Path>>(&self, path: P) -> bool {
        let path_str = path.as_ref().to_string_lossy();
        self.references.iter().any(|r| r.points_to(&path_str))
    }

    /// populates the given issues list with all duplicate sections in this document
    pub fn find_duplicate_sections(&self, path: &Path, issues: &mut Vec<Issue>) {
        // section title -> [lines with this section]
        let mut sections_lines: AHashMap<&str, Vec<(u32, u32, u32)>> = AHashMap::new();
        for section in self.sections() {
            sections_lines
                .entry(section.human_title())
                .or_insert_with(Vec::new)
                .push((
                    section.line_number,
                    section.title_text_start as u32,
                    section.title_text_end(),
                ));
        }
        for (title, lines) in sections_lines.drain() {
            if lines.len() > 1 {
                for (line, start, end) in lines {
                    issues.push(Issue::DuplicateSection {
                        location: Location {
                            file: path.into(),
                            line,
                            start,
                            end,
                        },
                        title: title.into(),
                    });
                }
            }
        }
    }

    /// populates the given issues list with all sections in this document that don't match the configured sections
    pub fn find_mismatching_footnotes(&self, path: &Path, issues: &mut Vec<Issue>) {
        let footnotes = match self.footnotes() {
            Ok(footnotes) => footnotes,
            Err(issue) => {
                issues.push(issue);
                return;
            }
        };
        for missing_reference in footnotes.missing_references() {
            issues.push(Issue::MissingFootnote {
                location: Location {
                    file: path.into(),
                    line: missing_reference.line,
                    start: missing_reference.start,
                    end: missing_reference.end,
                },
                identifier: missing_reference.identifier.clone(),
            });
        }
        for unused_definition in footnotes.unused_definitions() {
            issues.push(Issue::UnusedFootnote {
                location: Location {
                    file: path.into(),
                    line: unused_definition.line,
                    start: unused_definition.start,
                    end: unused_definition.end,
                },
                identifier: unused_definition.identifier.clone(),
            });
        }
    }

    /// populates the given issues list with all sections in this document that don't match the configured order
    pub fn find_unordered_sections(&self, path: &Path, config: &Config, issues: &mut Vec<Issue>) {
        let schema_titles = match &config.sections {
            None => return,
            Some(sections) => sections,
        };
        if self.content_sections.len() < 2 {
            // document has 0 or 1 sections --> order always matches
            return;
        }
        let mut doc_iter = self.content_sections.iter();
        let mut doc_section_option = doc_iter.next();
        let mut schema_iter = schema_titles.iter();
        let mut schema_title_option = schema_iter.next();
        loop {
            let doc_section = match doc_section_option {
                None => return, // we reached the end of the actual list --> actual matches schema
                Some(section) => section,
            };
            let schema_title = match schema_title_option {
                None => {
                    // end of schema reached but there are still unchecked sections in the document --> those are out of order
                    issues.push(Issue::UnorderedSections {
                        location: Location {
                            file: path.into(),
                            line: doc_section.line_number,
                            start: 0,
                            end: doc_section.title_line.text.len() as u32,
                        },
                    });
                    doc_section_option = doc_iter.next();
                    continue;
                }
                Some(value) => value,
            };
            let doc_section_title = doc_section.human_title();
            if doc_section_title == schema_title {
                // elements match --> advance both pointers
                doc_section_option = doc_iter.next();
                schema_title_option = schema_iter.next();
                continue;
            }
            // HACK: see https://github.com/rust-lang/rust/issues/42671
            if !schema_titles.iter().any(|st| st == doc_section_title) {
                // unknown element in actual --> ignore here (there is a separate check for this)
                doc_section_option = doc_iter.next();
                continue;
            }
            // elements don't match --> advance the schema
            // (because schema might contain elements that are not in actual)
            schema_title_option = schema_iter.next();
        }
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
                line.add_footnotes_to(&mut result, &self.relative_path, i as u32)?;
            }
        }
        if let Some(code_block_start) = code_block_start {
            return Err(Issue::UnclosedFence {
                location: Location {
                    file: self.relative_path.clone(),
                    line: code_block_start.line,
                    start: 0,
                    end: code_block_start.len,
                },
            });
        }
        Ok(result)
    }

    /// provides a Document instance containing the given text
    pub fn from_lines<T, P: Into<PathBuf>>(lines: T, relative_path: P) -> Result<Document, Issue>
    where
        T: Iterator<Item = String>,
    {
        let relative_path = relative_path.into();
        let mut sections: Vec<Section> = Vec::new();
        let mut section_builder: Option<section::Builder> = None;
        let mut inside_fence = false;
        let mut fence_line = 0;
        let mut old_occurrences_section: Option<Section> = None;
        for (line_number, line) in lines.enumerate() {
            if line.starts_with('#') && !inside_fence {
                if let Some(section_builder) = section_builder {
                    let section = section_builder.result();
                    if section.human_title() == "occurrences" {
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
                            file: relative_path,
                            line: line_number as u32,
                            start: 0,
                            end: line.len() as u32,
                        },
                    })
                }
            }
        }
        match section_builder {
            Some(section_builder) => {
                let section = section_builder.result();
                if section.human_title() == "occurrences" {
                    old_occurrences_section = Some(section);
                } else {
                    sections.push(section);
                }
            }
            None => {
                return Err(Issue::EmptyDocument {
                    path: relative_path,
                })
            }
        }
        if inside_fence {
            return Err(Issue::UnclosedFence {
                location: Location {
                    file: relative_path,
                    line: (fence_line as u32),
                    start: 0,
                    end: 0,
                },
            });
        }
        let mut sections = sections.into_iter();
        Ok(Document::new(
            relative_path,
            sections.next().unwrap(),
            sections.collect(),
            old_occurrences_section,
        ))
    }

    /// provides the Document contained in the file with the given path
    pub fn from_reader<R: BufRead, P: Into<PathBuf>>(
        reader: R,
        path: P,
    ) -> Result<Document, Issue> {
        let lines = reader.lines().map(Result::unwrap);
        Document::from_lines(lines, path)
    }

    #[cfg(test)]
    /// provides Document instances in tests
    pub fn from_str<P: Into<PathBuf>>(path: P, text: &str) -> Result<Document, Issue> {
        Document::from_lines(text.lines().map(std::string::ToString::to_string), path)
    }

    /// indicates whether this document contains the given anchor
    pub fn has_anchor(&self, anchor: &str) -> bool {
        self.content_sections
            .iter()
            .any(|section| section.anchor() == anchor)
    }

    /// provides the human-readable title of this document
    pub fn human_title(&self) -> &str {
        self.title_section.human_title()
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

    pub fn load<P: AsRef<Path>>(path: P, name: OsString) -> Result<Document, Issue> {
        let file = File::open(path.as_ref()).unwrap();
        Document::from_reader(BufReader::new(file), name)
    }

    pub fn new(
        path: PathBuf,
        title_section: Section,
        content_sections: Vec<Section>,
        old_occurrences_section: Option<Section>,
    ) -> Document {
        let mut references = vec![];
        Document::references(&title_section, &content_sections, &mut references);
        Document {
            relative_path: path,
            title_section,
            content_sections,
            old_occurrences_section,
            references,
        }
    }

    pub fn references(
        title_section: &Section,
        content_sections: &[Section],
        acc: &mut Vec<Reference>,
    ) {
        title_section.references(acc);
        for section in content_sections {
            section.references(acc);
        }
    }

    /// persists the changes made to this document to disk
    pub fn save(&self, root: &Path) {
        let mut file = fs::File::create(root.join(&self.relative_path)).unwrap();
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
            .map(Section::human_title)
            .collect()
    }

    /// provides the section with the given title
    pub fn section_with_title(&self, title: &str) -> Option<&Section> {
        self.content_sections
            .iter()
            .find(|section| section.human_title() == title)
    }

    /// provides the section with the given title
    pub fn section_with_title_mut(&mut self, title: &str) -> Option<&mut Section> {
        self.content_sections
            .iter_mut()
            .find(|section| section.human_title() == title)
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
    use crate::{Issue, Location};
    use indoc::indoc;
    use std::path::PathBuf;

    mod check_links {
        use crate::{test, Config, Issue, Location, Tikibase};
        use indoc::indoc;
        use std::path::PathBuf;

        #[test]
        fn link_to_non_existing_file() {
            let dir = test::tmp_dir();
            test::create_file("one.md", "# One\n\n[invalid](non-existing.md)\n", &dir);
            let base = Tikibase::load(dir).unwrap();
            let doc = base.get_doc("one.md").unwrap();
            let mut issues = vec![];
            let mut linked_resources = vec![];
            doc.check_links(
                &PathBuf::from("one.md"),
                &PathBuf::from(""),
                &mut issues,
                &mut linked_resources,
                &base.dir,
                &Config::default(),
            );
            let want = vec![Issue::LinkToNonExistingFile {
                location: Location {
                    file: "one.md".into(),
                    line: 2,
                    start: 0,
                    end: 26,
                },
                target: "non-existing.md".into(),
            }];
            pretty::assert_eq!(issues, want);
            assert_eq!(linked_resources, Vec::<PathBuf>::new());
        }

        #[test]
        fn link_to_non_existing_anchor_in_existing_file() {
            let dir = test::tmp_dir();
            test::create_file("1.md", "# One\n[non-existing anchor](2.md#zonk)\n", &dir);
            test::create_file("2.md", "# Two\n[One](1.md)", &dir);
            let base = Tikibase::load(dir).unwrap();
            let doc = base.get_doc("1.md").unwrap();
            let mut issues = vec![];
            let mut linked_resources = vec![];
            doc.check_links(
                &PathBuf::from("1.md"),
                &PathBuf::from(""),
                &mut issues,
                &mut linked_resources,
                &base.dir,
                &Config::default(),
            );
            let want = vec![Issue::LinkToNonExistingAnchorInExistingDocument {
                location: Location {
                    file: "1.md".into(),
                    line: 1,
                    start: 0,
                    end: 32,
                },
                target_file: "2.md".into(),
                anchor: "#zonk".into(),
            }];
            pretty::assert_eq!(issues, want);
            assert_eq!(linked_resources, Vec::<PathBuf>::new());
        }

        #[test]
        fn link_to_non_existing_anchor_in_current_file() {
            let dir = test::tmp_dir();
            test::create_file("1.md", "# One\n[non-existing anchor](#zonk)\n", &dir);
            let base = Tikibase::load(dir).unwrap();
            let doc = base.get_doc("1.md").unwrap();
            let mut issues = vec![];
            let mut linked_resources = vec![];
            doc.check_links(
                &PathBuf::from("1.md"),
                &PathBuf::from(""),
                &mut issues,
                &mut linked_resources,
                &base.dir,
                &Config::default(),
            );
            let want = vec![Issue::LinkToNonExistingAnchorInCurrentDocument {
                location: Location {
                    file: "1.md".into(),
                    line: 1,
                    start: 0,
                    end: 28,
                },
                anchor: "#zonk".into(),
            }];
            pretty::assert_eq!(issues, want);
            assert_eq!(linked_resources, Vec::<PathBuf>::new());
        }

        #[test]
        fn link_to_anchor_in_nonexisting_file() {
            let dir = test::tmp_dir();
            test::create_file(
                "1.md",
                "# One\n[anchor in non-existing file](2.md#foo)\n",
                &dir,
            );
            let base = Tikibase::load(dir).unwrap();
            let doc = base.get_doc("1.md").unwrap();
            let mut issues = vec![];
            let mut linked_resources = vec![];
            doc.check_links(
                &PathBuf::from("1.md"),
                &PathBuf::from(""),
                &mut issues,
                &mut linked_resources,
                &base.dir,
                &Config::default(),
            );
            let want = vec![Issue::LinkToNonExistingFile {
                location: Location {
                    file: "1.md".into(),
                    line: 1,
                    start: 0,
                    end: 39,
                },
                target: "2.md#foo".into(),
            }];
            pretty::assert_eq!(issues, want);
            assert_eq!(linked_resources, Vec::<PathBuf>::new());
        }

        #[test]
        fn link_to_existing_file() {
            let dir = test::tmp_dir();
            let content = indoc! {"
                # One
                working link to [Two](2.md)
                ### section
                working link to [Three](3.md)
                "};
            test::create_file("1.md", content, &dir);
            test::create_file("2.md", "# Two\n[1](1.md)", &dir);
            test::create_file("3.md", "# Three\n[1](1.md)", &dir);
            let base = Tikibase::load(dir).unwrap();
            let doc = base.get_doc("1.md").unwrap();
            let mut issues = vec![];
            let mut linked_resources = vec![];
            doc.check_links(
                &PathBuf::from("1.md"),
                &PathBuf::from(""),
                &mut issues,
                &mut linked_resources,
                &base.dir,
                &Config::default(),
            );
            pretty::assert_eq!(issues, vec![]);
            assert_eq!(linked_resources, Vec::<PathBuf>::new());
        }

        #[test]
        fn link_without_target() {
            let dir = test::tmp_dir();
            test::create_file("one.md", "# One\n\n[invalid]()\n", &dir);
            let base = Tikibase::load(dir).unwrap();
            let doc = base.get_doc("one.md").unwrap();
            let mut issues = vec![];
            let mut linked_resources = vec![];
            doc.check_links(
                &PathBuf::from("one.md"),
                &PathBuf::from(""),
                &mut issues,
                &mut linked_resources,
                &base.dir,
                &Config::default(),
            );
            pretty::assert_eq!(
                issues,
                vec![Issue::LinkWithoutTarget {
                    location: Location {
                        file: "one.md".into(),
                        line: 2,
                        start: 0,
                        end: 11,
                    }
                }]
            );
            assert_eq!(linked_resources, Vec::<PathBuf>::new());
        }

        #[test]
        fn link_to_external_url() {
            let dir = test::tmp_dir();
            let content = indoc! {"
                # One

                [external site](https://google.com)
                ![external image](https://google.com/foo.png)
                "};
            test::create_file("one.md", content, &dir);
            test::create_file("two.md", "# Two\n[one](one.md)", &dir);
            let base = Tikibase::load(dir).unwrap();
            let doc = base.get_doc("one.md").unwrap();
            let mut issues = vec![];
            let mut linked_resources = vec![];
            doc.check_links(
                &PathBuf::from("one.md"),
                &PathBuf::from(""),
                &mut issues,
                &mut linked_resources,
                &base.dir,
                &Config::default(),
            );
            assert!(issues.is_empty());
            assert_eq!(linked_resources, Vec::<PathBuf>::new());
        }

        #[test]
        fn imagelink_to_existing_image() {
            let dir = test::tmp_dir();
            test::create_file("1.md", "# One\n\n![image](foo.png)\n", &dir);
            test::create_file("foo.png", "image content", &dir);
            let base = Tikibase::load(dir).unwrap();
            let doc = base.get_doc("1.md").unwrap();
            let mut issues = vec![];
            let mut linked_resources = vec![];
            doc.check_links(
                &PathBuf::from("one.md"),
                &PathBuf::from(""),
                &mut issues,
                &mut linked_resources,
                &base.dir,
                &Config::default(),
            );
            assert!(issues.is_empty());
            assert_eq!(linked_resources, vec![PathBuf::from("foo.png")]);
        }

        #[test]
        fn imagelink_to_non_existing_image() {
            let dir = test::tmp_dir();
            test::create_file("1.md", "# One\n\n![image](zonk.png)\n", &dir);
            let base = Tikibase::load(dir).unwrap();
            let doc = base.get_doc("1.md").unwrap();
            let mut issues = vec![];
            let mut linked_resources = vec![];
            doc.check_links(
                &PathBuf::from("1.md"),
                &PathBuf::from(""),
                &mut issues,
                &mut linked_resources,
                &base.dir,
                &Config::default(),
            );
            let want = vec![Issue::BrokenImage {
                location: Location {
                    file: "1.md".into(),
                    line: 2,
                    start: 0,
                    end: 18,
                },
                target: "zonk.png".into(),
            }];
            pretty::assert_eq!(issues, want);
            assert_eq!(linked_resources, Vec::<PathBuf>::new());
        }

        #[test]
        fn link_to_existing_resource() {
            let dir = test::tmp_dir();
            test::create_file("1.md", "# One\n\n[docs](docs.pdf)\n", &dir);
            test::create_file("docs.pdf", "PDF content", &dir);
            let base = Tikibase::load(dir).unwrap();
            let doc = base.get_doc("1.md").unwrap();
            let mut issues = vec![];
            let mut linked_resources = vec![];
            doc.check_links(
                &PathBuf::from("1.md"),
                &PathBuf::from(""),
                &mut issues,
                &mut linked_resources,
                &base.dir,
                &Config::default(),
            );
            pretty::assert_eq!(issues, vec![]);
            assert_eq!(linked_resources, vec![PathBuf::from("docs.pdf")]);
        }
    }

    #[test]
    fn find_duplicate_sections() {
        let content = indoc! {"
            # test document

            ### One
            content
            ### One
            content"};
        let doc = Document::from_str("test.md", content).unwrap();
        let mut have = vec![];
        doc.find_duplicate_sections(&PathBuf::from("test.md"), &mut have);
        let want = vec![
            Issue::DuplicateSection {
                location: Location {
                    file: PathBuf::from("test.md"),
                    line: 2,
                    start: 4,
                    end: 7,
                },
                title: "One".into(),
            },
            Issue::DuplicateSection {
                location: Location {
                    file: PathBuf::from("test.md"),
                    line: 4,
                    start: 4,
                    end: 7,
                },
                title: "One".into(),
            },
        ];
        pretty::assert_eq!(have, want);
    }

    mod find_unused_footnotes {
        use crate::database::Document;
        use crate::{Issue, Location};
        use indoc::indoc;
        use std::path::PathBuf;

        #[test]
        fn missing_footnote_definition() {
            let content = indoc! {"
                # Title
                existing footnote[^existing]
                other footnote[^other]

                ```go
                result := map[^0]
                ```

                Another snippet of code that should be ignored: `map[^0]`.

                ### links

                [^existing]: existing footnote
                "};
            let doc = Document::from_str("test.md", content).unwrap();
            let mut have = vec![];
            doc.find_mismatching_footnotes(&PathBuf::from("test.md"), &mut have);
            let want = vec![Issue::MissingFootnote {
                location: Location {
                    file: PathBuf::from("test.md"),
                    line: 2,
                    start: 14,
                    end: 22,
                },
                identifier: "other".into(),
            }];
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn unused_footnote_definition() {
            let content = indoc! {"
                # Title
                existing footnote[^existing]

                ```go
                result := map[^0]
                ```

                Another snippet of code that should be ignored: `map[^0]`.

                ### links

                [^existing]: existing footnote
                [^unused]: unused footnote
                "};
            let doc = Document::from_str("test.md", content).unwrap();
            let mut have = vec![];
            doc.find_mismatching_footnotes(&PathBuf::from("test.md"), &mut have);
            let want = vec![Issue::UnusedFootnote {
                location: Location {
                    file: PathBuf::from("test.md"),
                    line: 12,
                    start: 0,
                    end: 10,
                },
                identifier: "unused".into(),
            }];
            pretty::assert_eq!(have, want);
        }
    }

    mod find_unordered_sections {
        use crate::database::Document;
        use crate::{Config, Issue, Location};
        use indoc::indoc;
        use std::path::PathBuf;

        #[test]
        fn mismatching() {
            let content = indoc! {"
            # Test
            ### one
            text
            ### three
            text
            ### two
            text"};
            let doc = Document::from_str("test.md", content).unwrap();
            let config = Config {
                sections: Some(vec!["one".into(), "two".into(), "three".into()]),
                ..Config::default()
            };
            let mut issues = vec![];
            doc.find_unordered_sections(&PathBuf::from("test.md"), &config, &mut issues);
            let want = vec![Issue::UnorderedSections {
                location: Location {
                    file: PathBuf::from("test.md"),
                    line: 5,
                    start: 0,
                    end: 7,
                },
            }];
            assert_eq!(issues, want);
        }

        #[test]
        fn perfect_match() {
            let content = indoc! {"
            # Test
            ### one
            text
            ### two
            text
            ### three
            text"};
            let doc = Document::from_str("test.md", content).unwrap();
            let config = Config {
                sections: Some(vec!["one".into(), "two".into(), "three".into()]),
                ..Config::default()
            };
            let mut issues = vec![];
            doc.find_unordered_sections(&PathBuf::from("test.md"), &config, &mut issues);
            let want = vec![];
            assert_eq!(issues, want);
        }

        #[test]
        fn match_but_missing() {
            let content = indoc! {"
            # Test
            ### one
            text
            ### two
            text
            ### three
            text"};
            let doc = Document::from_str("test.md", content).unwrap();
            let config = Config {
                sections: Some(vec!["one".into(), "three".into()]),
                ..Config::default()
            };
            let mut issues = vec![];
            doc.find_unordered_sections(&PathBuf::from("test.md"), &config, &mut issues);
            let want = vec![];
            assert_eq!(issues, want);
        }

        #[test]
        fn empty() {
            let content = indoc! {"
            # Test
            ### one
            text
            ### two
            text
            ### three
            text"};
            let doc = Document::from_str("test.md", content).unwrap();
            let config = Config {
                sections: None,
                ..Config::default()
            };
            let mut issues = vec![];
            doc.find_unordered_sections(&PathBuf::from("test.md"), &config, &mut issues);
            let want = vec![];
            assert_eq!(issues, want);
        }
    }

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
                relative_path: PathBuf::from("one.md"),
                title_section: Section {
                    line_number: 0,
                    title_line: Line::from("# test"),
                    body: vec![],
                    title_text_start: 2,
                    level: 1,
                },
                content_sections: vec![Section {
                    line_number: 1,
                    title_line: Line::from("### section 1"),
                    body: vec![Line::from("content")],
                    title_text_start: 4,
                    level: 3,
                }],
                old_occurrences_section: None,
                references: vec![],
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
                relative_path: PathBuf::from("test.md"),
                title_section: Section {
                    line_number: 0,
                    title_line: Line::from("# test"),
                    body: vec![
                        Line::from("```md"),
                        Line::from("### not a document section"),
                        Line::from("text"),
                        Line::from("```"),
                    ],
                    title_text_start: 2,
                    level: 1,
                },
                content_sections: vec![],
                old_occurrences_section: None,
                references: vec![],
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
                relative_path: PathBuf::from("one.md"),
                title_section: Section {
                    line_number: 0,
                    title_line: Line::from("# test"),
                    body: vec![],
                    title_text_start: 2,
                    level: 1,
                },
                content_sections: vec![
                    Section {
                        line_number: 1,
                        title_line: Line::from("### section 1"),
                        body: vec![Line::from("content")],
                        title_text_start: 4,
                        level: 3,
                    },
                    Section {
                        line_number: 5,
                        title_line: Line::from("### links"),
                        body: vec![Line::from("- link 1")],
                        title_text_start: 4,
                        level: 3,
                    },
                ],
                old_occurrences_section: Some(Section {
                    line_number: 3,
                    title_line: Line::from("### occurrences"),
                    body: vec![Line::from("- occurrence 1")],
                    title_text_start: 4,
                    level: 3,
                }),
                references: vec![],
            });
            pretty::assert_eq!(have, want);
        }
    }

    mod has_anchor {
        use crate::database::Document;

        #[test]
        fn matching() {
            let doc =
                Document::from_str("test.md", "# Title\n\n## head 1\ntext\n### head 2\n").unwrap();
            assert!(doc.has_anchor("#head-1"));
            assert!(doc.has_anchor("#head-2"));
            assert!(!doc.has_anchor("#head-3"));
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
                title_text_start: 4,
                level: 3,
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
                title_text_start: 2,
                level: 1,
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
        let text = indoc! {"
            # Title
            a link: [one](1.md)
            ### section
            an image: ![two](2.png)
            "};
        let doc = Document::from_str("test.md", text).unwrap();
        let mut have = vec![];
        Document::references(&doc.title_section, &doc.content_sections, &mut have);
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
        let have = doc.human_title();
        assert_eq!(have, "Title");
    }
}
