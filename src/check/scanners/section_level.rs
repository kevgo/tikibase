use crate::check::{Issue, Location};
use crate::database::Section;
use ahash::AHashMap;
use std::cmp::Ordering::{Equal, Greater, Less};

pub fn phase_1(section: &Section, level_variants: &mut AHashMap<String, AHashMap<u8, u32>>) {
  let entry = level_variants
    .entry(section.human_title().to_string())
    .or_default()
    .entry(section.level)
    .or_insert(0);
  *entry += 1;
}

/// converts the input to full tite --> `OutlierInfo`
pub fn find_outliers(input: AHashMap<String, AHashMap<u8, u32>>) -> AHashMap<String, OutlierInfo> {
  let mut result = AHashMap::new();
  for (title, variants) in input {
    let mut all: Vec<u8> = variants.keys().map(ToOwned::to_owned).collect();
    all.sort_unstable();
    match find_common_level(&variants) {
      Some(common) => {
        for (variant, _count) in variants {
          if variant != common {
            result.insert(
              format_variant(&title, variant),
              OutlierInfo {
                common: Some(common),
                all: all.clone(),
              },
            );
          }
        }
      }
      None => {
        for (variant, _count) in variants {
          result.insert(
            format_variant(&title, variant),
            OutlierInfo {
              common: None,
              all: all.clone(),
            },
          );
        }
      }
    }
  }
  result
}

pub fn phase_2(
  section: &Section,
  path: &str,
  issues: &mut Vec<Issue>,
  level_variants: &AHashMap<String, OutlierInfo>,
) {
  if let Some(outlier_info) = level_variants.get(&section.title_line.text) {
    issues.push(Issue::InconsistentHeadingLevel {
      location: Location {
        file: path.into(),
        line: section.line_number,
        start: 0,
        end: section.title_text_end(),
      },
      common_level: outlier_info.common,
      this_level: section.level,
      section_title: section.human_title().into(),
      all_levels: outlier_info.all.clone(),
    });
  }
}

/// provides the most common variant
fn find_common_level(level_counts: &AHashMap<u8, u32>) -> Option<u8> {
  let mut result = None;
  let mut max = 0;
  for (variant, count) in level_counts {
    match count.cmp(&max) {
      Greater => {
        result = Some(variant);
        max = count.to_owned();
      }
      Equal => {
        result = None;
      }
      Less => {}
    }
  }
  result.map(ToOwned::to_owned)
}

fn format_variant(title: &str, level: u8) -> String {
  format!("{} {}", "#".repeat(level as usize), title)
}

pub struct OutlierInfo {
  pub common: Option<u8>,
  pub all: Vec<u8>,
}

#[cfg(test)]
mod tests {

  mod scan {
    use crate::check::{Issue, Location};
    use crate::{test, Tikibase};
    use ahash::AHashMap;
    use big_s::S;
    use indoc::indoc;

    #[test]
    fn has_outlier() {
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
      let have = run(dir);
      let want = vec![Issue::InconsistentHeadingLevel {
        location: Location {
          file: S("2.md"),
          line: 2,
          start: 0,
          end: 13,
        },
        common_level: Some(3),
        this_level: 5u8,
        all_levels: vec![3, 5],
        section_title: S("section"),
      }];
      pretty::assert_eq!(have, want);
    }

    #[test]
    fn no_outlier() {
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
      let have = run(dir);
      let want = vec![
        Issue::InconsistentHeadingLevel {
          location: Location {
            file: S("1.md"),
            line: 2,
            start: 0,
            end: 11,
          },
          common_level: None,
          this_level: 3u8,
          all_levels: vec![3, 5],
          section_title: S("section"),
        },
        Issue::InconsistentHeadingLevel {
          location: Location {
            file: S("2.md"),
            line: 2,
            start: 0,
            end: 13,
          },
          common_level: None,
          this_level: 5u8,
          all_levels: vec![3, 5],
          section_title: S("section"),
        },
      ];
      pretty::assert_eq!(have, want);
    }

    #[test]
    fn no_problems() {
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
      let have = run(dir);
      let want = vec![];
      pretty::assert_eq!(have, want);
    }

    fn run(dir: String) -> Vec<Issue> {
      let base = Tikibase::load(dir).unwrap();
      // stage 1
      let mut title_variants = AHashMap::new();
      for (_filename, doc) in &base.dir.docs {
        for section in &doc.content_sections {
          super::super::phase_1(section, &mut title_variants);
        }
      }
      // stage 2
      let outliers = super::super::find_outliers(title_variants);
      // stage 3
      let mut issues = vec![];
      for (name, doc) in base.dir.docs {
        for section in doc.content_sections {
          super::super::phase_2(&section, &name, &mut issues, &outliers);
        }
      }
      issues.sort();
      issues
    }
  }

  mod find_most_common_level {
    use super::super::find_common_level;
    use ahash::AHashMap;

    #[test]
    fn has_outlier() {
      let mut give: AHashMap<u8, u32> = AHashMap::new();
      give.entry(3).or_insert(2);
      give.entry(4).or_insert(1);
      give.entry(5).or_insert(1);
      let have = find_common_level(&give);
      let want = Some(3);
      assert_eq!(have, want);
    }

    #[test]
    fn no_outlier() {
      let mut give: AHashMap<u8, u32> = AHashMap::new();
      give.entry(3).or_insert(1);
      give.entry(4).or_insert(1);
      give.entry(5).or_insert(1);
      let have = find_common_level(&give);
      let want = None;
      assert_eq!(have, want);
    }

    #[test]
    fn no_problems() {
      let mut give: AHashMap<u8, u32> = AHashMap::new();
      give.entry(3).or_insert(2);
      let have = find_common_level(&give);
      let want = Some(3);
      assert_eq!(have, want);
    }
  }
}
