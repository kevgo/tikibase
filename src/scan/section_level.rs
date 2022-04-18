use crate::{Issue, Location, Tikibase};
use ahash::AHashMap;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::path::Path;

pub(crate) fn scan(base: &Tikibase) -> Vec<Issue> {
    // title --> level --> FileSections with this level and title
    let mut level_variants: AHashMap<&str, AHashMap<u8, Vec<FileSection>>> = AHashMap::new();
    for doc in &base.dir.docs {
        for section in doc.sections() {
            level_variants
                .entry(section.human_title())
                .or_insert_with(AHashMap::new)
                .entry(section.level)
                .or_insert_with(Vec::new)
                .push(FileSection {
                    file: &doc.relative_path,
                    title: section.human_title(),
                    line: section.line_number,
                    end: section.title_text_end(),
                });
        }
    }
    let mut issues = vec![];
    for (_, level_counts) in level_variants.drain() {
        if level_counts.len() < 2 {
            // one type of level --> section is consistently formatted everywhere
            continue;
        }
        let common_level = find_common_level(&level_counts);
        let mut all_variants: Vec<u8> = level_counts.keys().map(ToOwned::to_owned).collect();
        all_variants.sort_unstable();
        for (level, file_sections) in level_counts {
            if let Some(common_level) = common_level {
                if level == common_level {
                    continue;
                }
            }
            for file_section in file_sections {
                issues.push(Issue::InconsistentHeadingLevel {
                    location: Location {
                        file: file_section.file.into(),
                        line: file_section.line,
                        start: 0,
                        end: file_section.end,
                    },
                    common_level,
                    this_level: level as u8,
                    section_title: file_section.title.into(),
                    all_levels: all_variants.clone(),
                });
            }
        }
    }
    issues.sort();
    issues
}

#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FileSection<'a> {
    pub file: &'a Path,
    pub title: &'a str,
    pub line: u32,
    pub end: u32,
}

impl Default for FileSection<'_> {
    fn default() -> Self {
        Self {
            file: Path::new(""),
            title: "",
            line: 0,
            end: 0,
        }
    }
}

/// provides the most common key
fn find_common_level(level_counts: &AHashMap<u8, Vec<FileSection>>) -> Option<u8> {
    let mut result = None;
    let mut max = 0;
    for (name, elements) in level_counts {
        match elements.len().cmp(&max) {
            Greater => {
                result = Some(name.to_owned());
                max = elements.len();
            }
            Equal => {
                result = None;
            }
            Less => {}
        }
    }
    result
}

#[cfg(test)]
mod tests {

    mod scan {
        use crate::{test, Config, Issue, Location, Tikibase};
        use indoc::indoc;
        use std::path::PathBuf;

        #[test]
        fn outlier_level() {
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
            let have = super::super::scan(&base);
            let want = vec![Issue::InconsistentHeadingLevel {
                location: Location {
                    file: PathBuf::from("2.md"),
                    line: 2,
                    start: 0,
                    end: 13,
                },
                common_level: Some(3),
                this_level: 5u8,
                all_levels: vec![3, 5],
                section_title: "section".into(),
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
            let have = super::super::scan(&base);
            let want = vec![
                Issue::InconsistentHeadingLevel {
                    location: Location {
                        file: PathBuf::from("1.md"),
                        line: 2,
                        start: 0,
                        end: 11,
                    },
                    common_level: None,
                    this_level: 3u8,
                    all_levels: vec![3, 5],
                    section_title: "section".into(),
                },
                Issue::InconsistentHeadingLevel {
                    location: Location {
                        file: PathBuf::from("2.md"),
                        line: 2,
                        start: 0,
                        end: 13,
                    },
                    common_level: None,
                    this_level: 5u8,
                    all_levels: vec![3, 5],
                    section_title: "section".into(),
                },
            ];
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn matching_levels() {
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
            let have = super::super::scan(&base);
            let want = vec![];
            pretty::assert_eq!(have, want);
        }
    }

    mod find_most_common_level {
        use super::super::{find_common_level, FileSection};
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
            let have = find_common_level(&give);
            let want = Some(3);
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
            let have = find_common_level(&give);
            let want = None;
            assert_eq!(have, want);
        }
    }
}
