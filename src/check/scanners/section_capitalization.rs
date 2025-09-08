use crate::check::{Issue, Location};
use crate::database::Section;
use ahash::AHashMap;
use core::cmp::Ordering::{Equal, Greater, Less};

pub fn phase_1(section: &Section, title_variants: &mut AHashMap<String, u32>) {
  let entry = title_variants
    .entry(section.human_title().to_owned())
    .or_insert(0);
  *entry += 1;
}

pub fn find_outliers(mut input: AHashMap<String, u32>) -> AHashMap<String, OutlierInfo> {
  // step 1: group related variants together
  // normalized variant --> variant --> count
  let mut grouped: AHashMap<String, AHashMap<String, u32>> = AHashMap::new();
  for (variant, count) in input.drain() {
    grouped
      .entry(variant.to_lowercase())
      .or_default()
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

pub fn phase_2(
  path: &str,
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

/// information about an unusual section capitalization
pub struct OutlierInfo {
  /// the most common capitalization variant
  pub common: Option<String>,
  /// all capitalization variants
  pub all: Vec<String>,
}

/// provides the most common variant within the given capitalization variants
fn find_common_capitalization(variants: &AHashMap<String, u32>) -> Option<String> {
  let mut max_count = 0;
  let mut max_variant = None;
  for (variant, count) in variants {
    match count.cmp(&max_count) {
      Greater => {
        count.clone_into(&mut max_count);
        max_variant = Some(variant);
      }
      Equal => {
        max_variant = None;
      }
      Less => {}
    }
  }
  max_variant.map(ToString::to_string)
}

#[cfg(test)]
mod tests {
  use crate::check::{Issue, Location};
  use crate::{Tikibase, test};
  use ahash::AHashMap;
  use big_s::S;
  use camino::Utf8PathBuf;
  use indoc::indoc;

  #[test]
  fn has_common_capitalization() {
    // create files
    let dir = camino_tempfile::tempdir().unwrap();
    let content1 = indoc! {"
            # One

            ### alpha
            [2](2.md)"};
    test::create_file("1.md", content1, dir.path());
    let content2 = indoc! {"
            # Two

            ### Alpha
            [3](3.md)"};
    test::create_file("2.md", content2, dir.path());
    let content3 = indoc! {"
            # Three

            ### alpha
            [1](1.md)"};
    test::create_file("3.md", content3, dir.path());
    let have = run(dir.path());
    let want = vec![Issue::MixCapSection {
      location: Location {
        file: S("2.md"),
        line: 2,
        start: 4,
        end: 9,
      },
      all_variants: vec![S("Alpha"), S("alpha")],
      this_variant: S("Alpha"),
      common_variant: Some(S("alpha")),
      section_level: 3,
    }];
    pretty::assert_eq!(have, want);
  }

  #[test]
  fn mixed_capitalization_same_counts() {
    let dir = camino_tempfile::tempdir().unwrap();
    let content1 = indoc! {"
            # One

            ### alpha
            [2](2.md)"};
    test::create_file("1.md", content1, dir.path());
    let content2 = indoc! {"
            # Two

            ### Alpha
            [1](1.md)"};
    test::create_file("2.md", content2, dir.path());
    let have = run(dir.path());
    let want = vec![
      Issue::MixCapSection {
        location: Location {
          file: S("1.md"),
          line: 2,
          start: 4,
          end: 9,
        },
        all_variants: vec![S("Alpha"), S("alpha")],
        this_variant: S("alpha"),
        common_variant: None,
        section_level: 3,
      },
      Issue::MixCapSection {
        location: Location {
          file: S("2.md"),
          line: 2,
          start: 4,
          end: 9,
        },
        all_variants: vec![S("Alpha"), S("alpha")],
        this_variant: S("Alpha"),
        common_variant: None,
        section_level: 3,
      },
    ];
    pretty::assert_eq!(have, want);
  }

  #[test]
  fn same_capitalization() {
    let dir = camino_tempfile::tempdir().unwrap();
    let content1 = indoc! {"
            # One

            ### alpha
            [2](2.md)"};
    test::create_file("1.md", content1, dir.path());
    let content2 = indoc! {"
            # Two

            ### alpha
            [1](1.md)"};
    test::create_file("2.md", content2, dir.path());
    let have = run(dir.path());
    let want = vec![];
    pretty::assert_eq!(have, want);
  }

  fn run<P: Into<Utf8PathBuf>>(dir: P) -> Vec<Issue> {
    let base = Tikibase::load(dir.into()).unwrap();
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
        super::phase_2(&name, &section, &mut issues, &outliers);
      }
    }
    issues.sort();
    issues
  }
}
