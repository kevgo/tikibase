use crate::{Issue, Location, Tikibase};
use ahash::AHashMap;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::path::Path;

pub(crate) fn scan(base: &Tikibase) -> Vec<Issue> {
    // normalized title --> variant --> sections with this variant
    let mut title_variants: AHashMap<String, AHashMap<&str, Vec<FileSection>>> = AHashMap::new();
    for doc in &base.docs {
        for section in doc.sections() {
            let section_title = section.human_title();
            title_variants
                .entry(normalize(section_title))
                .or_insert_with(AHashMap::new)
                .entry(section_title)
                .or_insert_with(Vec::new)
                .push(FileSection {
                    title: section_title,
                    file: &doc.relative_path,
                    level: section.level,
                    line: section.line_number,
                    start: section.title_text_start as u32,
                });
        }
    }
    let mut issues = Vec::new();
    for (_, variants_sections) in title_variants.drain() {
        if variants_sections.len() < 2 {
            // one type of capitalization --> section is consistently formatted everywhere
            continue;
        }
        let common_variant = find_common_capitalization(&variants_sections);
        let mut all_variants: Vec<String> =
            variants_sections.keys().map(ToString::to_string).collect();
        all_variants.sort_unstable();
        for (variant, file_sections) in variants_sections {
            if let Some(common_variant) = &common_variant {
                if variant == common_variant {
                    continue;
                }
            }
            for file_section in file_sections {
                issues.push(Issue::MixCapSection {
                    location: Location {
                        file: file_section.file.into(),
                        line: file_section.line,
                        start: file_section.start,
                        end: file_section.end(),
                    },
                    all_variants: all_variants.clone(),
                    this_variant: variant.into(),
                    common_variant: common_variant.clone(),
                    section_level: file_section.level,
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
    pub level: u8,
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
            level: 1,
        }
    }
}

/// provides the most common key
fn find_common_capitalization(level_counts: &AHashMap<&str, Vec<FileSection>>) -> Option<String> {
    let mut result: Option<&str> = None;
    let mut max = 0;
    for (variant, file_sections) in level_counts {
        match file_sections.len().cmp(&max) {
            Greater => {
                result = Some(*variant);
                max = file_sections.len();
            }
            Equal => {
                result = None;
            }
            Less => {}
        }
    }
    result.map(ToString::to_string)
}

/// normalizes the given section title
fn normalize(section_title: &str) -> String {
    section_title.to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use crate::{test, Config, Tikibase};
    use indoc::indoc;

    mod scan {
        use crate::{test, Config, Issue, Location, Tikibase};
        use indoc::indoc;
        use std::path::PathBuf;

        #[test]
        fn outlier_capitalization() {
            let dir = test::tmp_dir();
            let content1 = indoc! {"
            # One

            ### alpha
            [2](2.md)"};
            test::create_file("1.md", content1, &dir);
            let content2 = indoc! {"
            # Two

            ### Alpha
            [3](3.md)"};
            test::create_file("2.md", content2, &dir);
            let content3 = indoc! {"
            # Three

            ### alpha
            [1](1.md)"};
            test::create_file("3.md", content3, &dir);
            let base = Tikibase::load(dir, &Config::default()).unwrap();
            let have = super::super::scan(&base);
            let want = vec![Issue::MixCapSection {
                location: Location {
                    file: PathBuf::from("2.md"),
                    line: 2,
                    start: 4,
                    end: 9,
                },
                all_variants: vec!["Alpha".into(), "alpha".into()],
                this_variant: "Alpha".into(),
                common_variant: Some("alpha".into()),
                section_level: 3,
            }];
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn mixed_capitalization_same_counts() {
            let dir = test::tmp_dir();
            let content1 = indoc! {"
            # One

            ### alpha
            [2](2.md)"};
            test::create_file("1.md", content1, &dir);
            let content2 = indoc! {"
            # Two

            ### Alpha
            [1](1.md)"};
            test::create_file("2.md", content2, &dir);
            let base = Tikibase::load(dir, &Config::default()).unwrap();
            let have = super::super::scan(&base);
            let want = vec![
                Issue::MixCapSection {
                    location: Location {
                        file: PathBuf::from("1.md"),
                        line: 2,
                        start: 4,
                        end: 9,
                    },
                    all_variants: vec!["Alpha".into(), "alpha".into()],
                    this_variant: "alpha".into(),
                    common_variant: None,
                    section_level: 3,
                },
                Issue::MixCapSection {
                    location: Location {
                        file: PathBuf::from("2.md"),
                        line: 2,
                        start: 4,
                        end: 9,
                    },
                    all_variants: vec!["Alpha".into(), "alpha".into()],
                    this_variant: "Alpha".into(),
                    common_variant: None,
                    section_level: 3,
                },
            ];
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn same_capitalization() {
            let dir = test::tmp_dir();
            let content1 = indoc! {"
            # One

            ### alpha
            [2](2.md)"};
            test::create_file("1.md", content1, &dir);
            let content2 = indoc! {"
            # Two

            ### alpha
            [1](1.md)"};
            test::create_file("2.md", content2, &dir);
            let base = Tikibase::load(dir, &Config::default()).unwrap();
            let have = super::super::scan(&base);
            let want = vec![];
            pretty::assert_eq!(have, want);
        }
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
}
