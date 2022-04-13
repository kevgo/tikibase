use crate::{Issue, Location, Tikibase};
use ahash::{AHashMap, AHashSet};
use std::path::Path;

pub(crate) fn scan(base: &Tikibase) -> Vec<Issue> {
    // registers variants of section titles: normalized title --> Vec<sections with a variation of this title>
    let mut title_variants: AHashMap<String, Vec<FileSection>> = AHashMap::new();
    for doc in &base.docs {
        for section in doc.sections() {
            let section_title = section.human_title();
            title_variants
                .entry(normalize(section_title))
                .or_insert_with(Vec::new)
                .push(FileSection {
                    title: section_title,
                    file: &doc.path,
                    line: section.line_number,
                    start: section.title_text_start as u32,
                });
        }
    }
    let mut issues = Vec::new();
    for (_, file_sections) in title_variants.drain() {
        if variants_count(&file_sections) < 2 {
            continue;
        }
        // remove duplicates
        let variants: AHashSet<String> = file_sections
            .iter()
            .map(|variant| variant.title.into())
            .collect();
        // sort
        let mut variants: Vec<String> = Vec::from_iter(variants);
        variants.sort();
        for file_section in file_sections {
            issues.push(Issue::MixCapSection {
                variants: variants.clone(),
                location: Location {
                    file: file_section.file.into(),
                    line: file_section.line,
                    start: file_section.start,
                    end: file_section.end(),
                },
            });
        }
    }
    issues
}

#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FileSection<'a> {
    pub file: &'a Path,
    pub title: &'a str,
    pub line: u32,
    pub start: u32,
}

impl FileSection<'_> {
    pub fn end(&self) -> u32 {
        self.start + self.title.len() as u32
    }
}

impl Default for FileSection<'_> {
    fn default() -> Self {
        Self {
            file: Path::new(""),
            title: "",
            line: 0,
            start: 0,
        }
    }
}

fn variants_count(file_sections: &[FileSection]) -> usize {
    let set: AHashSet<&str> = file_sections.iter().map(|fs| fs.title).collect();
    set.len()
}

/// normalizes the given section title
fn normalize(section_title: &str) -> String {
    section_title.to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::FileSection;
    use crate::{test, Config, Issue, Location, Tikibase};
    use indoc::indoc;
    use std::path::PathBuf;

    #[test]
    fn different_capitalization() {
        let dir = test::tmp_dir();
        let content1 = indoc! {"
            # test document

            ### ONE
            content

            ### One
            content"};
        test::create_file("1.md", content1, &dir);
        let content2 = indoc! {"
            # another document

            ### one
            content

            ### ONE
            content"};
        test::create_file("2.md", content2, &dir);
        let base = Tikibase::load(dir, &Config::default()).unwrap();
        let have = super::scan(&base);
        let want = vec![
            Issue::MixCapSection {
                location: Location {
                    file: PathBuf::from("1.md"),
                    line: 2,
                    start: 4,
                    end: 7,
                },
                variants: vec!["ONE".into(), "One".into(), "one".into()],
            },
            Issue::MixCapSection {
                location: Location {
                    file: PathBuf::from("1.md"),
                    line: 5,
                    start: 4,
                    end: 7,
                },
                variants: vec!["ONE".into(), "One".into(), "one".into()],
            },
            Issue::MixCapSection {
                location: Location {
                    file: PathBuf::from("2.md"),
                    line: 2,
                    start: 4,
                    end: 7,
                },
                variants: vec!["ONE".into(), "One".into(), "one".into()],
            },
            Issue::MixCapSection {
                location: Location {
                    file: PathBuf::from("2.md"),
                    line: 5,
                    start: 4,
                    end: 7,
                },
                variants: vec!["ONE".into(), "One".into(), "one".into()],
            },
        ];
        pretty::assert_eq!(have, want);
    }

    mod file_section {
        use crate::scan::section_capitalization::FileSection;

        #[test]
        fn end() {
            let file_section = FileSection {
                title: "test section",
                start: 4,
                ..FileSection::default()
            };
            assert_eq!(file_section.end(), 16);
        }
    }

    #[test]
    fn normalize() {
        assert_eq!(super::normalize("foo"), "foo");
        assert_eq!(super::normalize("Foo"), "foo");
        assert_eq!(super::normalize("FOO"), "foo");
    }

    #[test]
    fn same_capitalization() {
        let dir = test::tmp_dir();
        let content1 = indoc! {"
            # test document

            ### One
            content

            ### One
            content"};
        test::create_file("1.md", content1, &dir);
        let base = Tikibase::load(dir, &Config::default()).unwrap();
        let have = super::scan(&base);
        let want = vec![];
        pretty::assert_eq!(have, want);
    }

    #[test]
    fn variants_count() {
        let give: Vec<FileSection> = vec![
            FileSection {
                title: "One",
                ..FileSection::default()
            },
            FileSection {
                title: "One",
                ..FileSection::default()
            },
            FileSection {
                title: "one",
                ..FileSection::default()
            },
        ];
        let have = super::variants_count(&give);
        let want = 2;
        assert_eq!(have, want);
    }
}
