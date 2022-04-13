use crate::{Issue, Location, Tikibase};
use ahash::AHashMap;
use std::path::Path;

pub(crate) fn scan(base: &Tikibase) -> Vec<Issue> {
    // title --> level --> FileSections with this level and title
    let mut level_variants: AHashMap<&str, AHashMap<u8, Vec<FileSection>>> = AHashMap::new();
    for doc in &base.docs {
        for section in doc.sections() {
            let section_title = section.human_title();
            level_variants
                .entry(section_title)
                .or_insert_with(AHashMap::new)
                .entry(section.level)
                .or_insert_with(Vec::new)
                .push(FileSection {
                    file: &doc.path,
                    title: section_title,
                    line: section.line_number,
                    start: section.title_text_start as u32,
                })
        }
    }
    let mut issues = vec![];
    for (_, level_counts) in level_variants.drain() {
        if level_counts.len() < 2 {
            continue;
        }
        let most_common_levels = find_most_common_levels(&level_counts);
        for (level, file_sections) in level_counts {
            if most_common_levels.contains(&level) {
                continue;
            }
            for file_section in file_sections {
                issues.push(Issue::InconsistentHeadingLevel {
                    location: Location {
                        file: file_section.file.into(),
                        line: file_section.line,
                        start: file_section.start,
                        end: file_section.end(),
                    },
                    common_variants: most_common_levels.clone(),
                    this_variant: level as u8,
                })
            }
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

/// Provides the key with the most elements, if one can be determined.
/// Returns None if there are multiple
fn find_most_common_levels(level_counts: &AHashMap<u8, Vec<FileSection>>) -> Vec<u8> {
    let mut result = vec![];
    let mut max = 0;
    for (name, elements) in level_counts {
        let count = elements.len();
        if count > max {
            result = vec![name.to_owned()];
            max = count;
        } else if count == max {
            result.push(name.to_owned());
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::{test, Config, Issue, Location, Tikibase};
    use indoc::indoc;
    use std::path::PathBuf;

    #[test]
    fn different_levels_different_counts() {
        let dir = test::tmp_dir();
        let content1 = indoc! {"
            # one

            ### section
            content"};
        test::create_file("1.md", content1, &dir);
        let content2 = indoc! {"
            # two

            ##### section
            content"};
        test::create_file("2.md", content2, &dir);
        let content3 = indoc! {"
            # three

            ### section
            content"};
        test::create_file("3.md", content3, &dir);
        let base = Tikibase::load(dir, &Config::default()).unwrap();
        let have = super::scan(&base);
        let want = vec![Issue::InconsistentHeadingLevel {
            location: Location {
                file: PathBuf::from("2.md"),
                line: 2,
                start: 6,
                end: 13,
            },
            common_variants: vec![3],
            this_variant: 5u8,
        }];
        pretty::assert_eq!(have, want);
    }

    #[test]
    fn different_levels_same_counts() {
        let dir = test::tmp_dir();
        let content1 = indoc! {"
            # one

            ### section
            content"};
        test::create_file("1.md", content1, &dir);
        let content2 = indoc! {"
            # two

            ##### section
            content"};
        test::create_file("2.md", content2, &dir);
        let base = Tikibase::load(dir, &Config::default()).unwrap();
        let have = super::scan(&base);
        let want = vec![
            Issue::InconsistentHeadingLevel {
                location: Location {
                    file: PathBuf::from("1.md"),
                    line: 2,
                    start: 3,
                    end: 10,
                },
                common_variants: vec![5],
                this_variant: 3u8,
            },
            Issue::InconsistentHeadingLevel {
                location: Location {
                    file: PathBuf::from("2.md"),
                    line: 2,
                    start: 6,
                    end: 13,
                },
                common_variants: vec![3],
                this_variant: 5u8,
            },
        ];
        pretty::assert_eq!(have, want);
    }

    #[test]
    fn same_levels() {
        let dir = test::tmp_dir();
        let content1 = indoc! {"
            # one

            ### section
            content"};
        test::create_file("1.md", content1, &dir);
        let content2 = indoc! {"
            # two

            ### section
            content"};
        test::create_file("2.md", content2, &dir);
        let base = Tikibase::load(dir, &Config::default()).unwrap();
        let have = super::scan(&base);
        let want = vec![];
        pretty::assert_eq!(have, want);
    }

    mod file_section {
        use super::super::FileSection;

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

    mod find_most_common_level {
        use super::super::{find_most_common_levels, FileSection};
        use ahash::AHashMap;

        #[test]
        fn different_counts() {
            let mut give: AHashMap<u8, Vec<FileSection>> = AHashMap::new();
            give.entry(3).or_insert_with(Vec::new).push(FileSection {
                title: "3A",
                ..FileSection::default()
            });
            give.entry(3).or_insert_with(Vec::new).push(FileSection {
                title: "3B",
                ..FileSection::default()
            });
            give.entry(5).or_insert_with(Vec::new).push(FileSection {
                title: "5A",
                ..FileSection::default()
            });
            let have = find_most_common_levels(&give);
            let want = vec![3];
            assert_eq!(have, want);
        }

        #[test]
        fn same_counts() {
            let mut give: AHashMap<u8, Vec<FileSection>> = AHashMap::new();
            give.entry(3).or_insert_with(Vec::new).push(FileSection {
                title: "3A",
                ..FileSection::default()
            });
            give.entry(5).or_insert_with(Vec::new).push(FileSection {
                title: "5A",
                ..FileSection::default()
            });
            let have = find_most_common_levels(&give);
            let want = vec![3, 5];
            assert_eq!(have, want);
        }
    }
}
