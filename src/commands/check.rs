use super::Outcome;
use crate::check::scanners::{section_capitalization, section_level};
use crate::check::{dir_phase_1, dir_phase_2, State1, State2};
use crate::Tikibase;

#[must_use]
pub fn check(base: &Tikibase) -> Outcome {
  let mut state_1 = State1::empty(&base.dir);
  dir_phase_1(&base.dir, "", &mut state_1);
  let mut state_2 = State2 {
    capitalization_outliers: section_capitalization::find_outliers(state_1.capitalization_variants),
    level_outliers: section_level::find_outliers(state_1.level_variants),
    linked_resources: state_1.linked_resources,
    issues: state_1.issues,
  };
  dir_phase_2(&base.dir, &mut state_2);
  state_2.issues.sort();
  Outcome {
    issues: state_2.issues,
    fixes: vec![],
  }
}

#[cfg(test)]
mod tests {
  use crate::check::{Issue, Location};
  use crate::commands::Outcome;
  use crate::{test, Tikibase};
  use big_s::S;

  #[test]
  fn missing_links() {
    let dir = test::tmp_dir();
    test::create_file("1.md", "# One\n\ntext\n", &dir);
    test::create_file("2.md", "# Two\n\n[one](1.md)\n", &dir);
    test::create_file("3.md", "# Three\n\n[one](1.md)\n", &dir);
    test::create_file("tikibase.json", r#"{ "bidiLinks": true }"#, &dir);
    let base = Tikibase::load(dir).unwrap();
    let have = super::check(&base);
    let want = Outcome {
      issues: vec![
        Issue::DocumentWithoutLinks {
          location: Location {
            file: S("1.md"),
            line: 0,
            start: 0,
            end: 0,
          },
        },
        Issue::MissingLink {
          location: Location {
            file: S("1.md"),
            line: 2,
            start: 0,
            end: 0,
          },
          path: S("2.md"),
          title: S("Two"),
        },
        Issue::MissingLink {
          location: Location {
            file: S("1.md"),
            line: 2,
            start: 0,
            end: 0,
          },
          path: S("3.md"),
          title: S("Three"),
        },
      ],
      fixes: vec![],
    };
    pretty::assert_eq!(have, want);
  }

  #[test]
  fn obsolete_occurrences() {
    let dir = test::tmp_dir();
    test::create_file("1.md", "# One\n\ntext\n### occurrences\n\n- foo", &dir);
    test::create_file("tikibase.json", r#"{ "bidiLinks": true }"#, &dir);
    let base = Tikibase::load(dir).unwrap();
    let have = super::check(&base);
    let want = Outcome {
      issues: vec![
        Issue::DocumentWithoutLinks {
          location: Location {
            file: S("1.md"),
            line: 0,
            start: 0,
            end: 0,
          },
        },
        Issue::ObsoleteOccurrencesSection {
          location: Location {
            file: S("1.md"),
            line: 3,
            start: 4,
            end: 15,
          },
        },
      ],
      fixes: vec![],
    };
    pretty::assert_eq!(have, want);
  }
}
