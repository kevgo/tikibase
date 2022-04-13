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
        let most_common_level = find_most_common_level(&level_counts);
        for (level, file_sections) in level_counts {
            for file_section in file_sections {
                issues.push(Issue::InconsistentHeadingLevel {
                    location: Location {
                        file: file_section.file.into(),
                        line: file_section.line,
                        start: file_section.start,
                        end: file_section.end(),
                    },
                    common_variant: most_common_level as u8,
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

fn find_most_common_level(level_counts: &AHashMap<u8, Vec<FileSection>>) -> u8 {
    level_counts.keys().max().unwrap().to_owned()
}

#[cfg(test)]
mod tests {
    use crate::{test, Config, Issue, Location, Tikibase};
    use indoc::indoc;
    use std::path::PathBuf;

    #[test]
    fn different_levels() {
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
            common_variant: 3u8,
            this_variant: 5u8,
        }];
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
}
