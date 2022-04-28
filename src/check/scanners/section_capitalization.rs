use crate::check::{Issue, Location};
use crate::database::Section;
use ahash::AHashMap;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::path::Path;

pub(crate) fn phase_1(section: &Section, title_variants: &mut AHashMap<String, u32>) {
    let entry = title_variants
        .entry(section.human_title().to_string())
        .or_insert(0);
    *entry += 1;
}

pub(crate) fn find_outliers(mut input: AHashMap<String, u32>) -> AHashMap<String, OutlierInfo> {
    // step 1: group related variants together
    // normalized variant --> variant --> count
    let mut grouped: AHashMap<String, AHashMap<String, u32>> = AHashMap::new();
    for (variant, count) in input.drain() {
        grouped
            .entry(variant.to_lowercase())
            .or_insert_with(AHashMap::new)
            .insert(variant, count);
    }
    // step 2: find the outliers
    let mut outliers = AHashMap::new();
    for (_, variants) in grouped {
        let mut all: Vec<String> = variants.keys().map(ToString::to_string).collect();
        all.sort_unstable();
        match find_common_capitalization(&variants) {
            Some(common) => {
                for (variant, _count) in variants {
                    if variant != common {
                        outliers.insert(
                            variant,
                            OutlierInfo {
                                common: Some(common.clone()),
                                all: all.clone(),
                            },
                        );
                    }
                }
            }
            None => {
                for (variant, _count) in variants {
                    outliers.insert(
                        variant,
                        OutlierInfo {
                            common: None,
                            all: all.clone(),
                        },
                    );
                }
            }
        }
    }
    outliers
}

pub(crate) fn phase_2(
    path: &Path,
    section: &Section,
    issues: &mut Vec<Issue>,
    outliers: &AHashMap<String, OutlierInfo>,
) {
    let section_title = section.human_title();
    if let Some(outlier_info) = outliers.get(section_title) {
        issues.push(Issue::MixCapSection {
            location: Location {
                file: path.into(),
                line: section.line_number,
                start: section.title_text_start as u32,
                end: section.title_text_end(),
            },
            all_variants: outlier_info.all.clone(),
            this_variant: section_title.into(),
            common_variant: outlier_info.common.clone(),
            section_level: section.level,
        });
    }
}

pub struct OutlierInfo {
    pub common: Option<String>,
    pub all: Vec<String>,
}

/// provides the most common key
fn find_common_capitalization(variants: &AHashMap<String, u32>) -> Option<String> {
    let mut max_count = 0;
    let mut max_variant = "".to_string();
    let mut unique = true;
    for (variant, count) in variants {
        match count.cmp(&max_count) {
            Greater => {
                max_count = count.to_owned();
                max_variant = variant.into();
                unique = true;
            }
            Equal => {
                unique = false;
            }
            Less => {}
        }
    }
    if unique {
        Some(max_variant)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {

    use crate::check::{Issue, Location};
    use crate::{test, Tikibase};
    use ahash::AHashMap;
    use indoc::indoc;
    use std::path::PathBuf;

    #[test]
    fn outlier_capitalization() {
        // create files
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
        let have = run(dir);
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
        let have = run(dir);
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
        let have = run(dir);
        let want = vec![];
        pretty::assert_eq!(have, want);
    }

    fn run(dir: PathBuf) -> Vec<Issue> {
        let base = Tikibase::load(dir).unwrap();
        // stage 1
        let mut title_variants = AHashMap::new();
        for (_filename, doc) in &base.dir.docs {
            for section in &doc.content_sections {
                super::phase_1(section, &mut title_variants);
            }
        }
        // stage 2
        let outliers = super::find_outliers(title_variants);
        // stage 3
        let mut issues = vec![];
        for (name, doc) in base.dir.docs {
            for section in doc.content_sections {
                super::phase_2(
                    &PathBuf::new().join(&name),
                    &section,
                    &mut issues,
                    &outliers,
                );
            }
        }
        issues.sort();
        issues
    }
}
