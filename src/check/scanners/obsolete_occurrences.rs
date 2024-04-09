use crate::check::{Issue, Location};
use crate::database::Document;
use crate::Config;

pub fn scan(doc: &Document, config: &Config, issues: &mut Vec<Issue>) {
  if let Some(bidi_links) = config.bidi_links {
    if let Some(old_occurrences_section) = &doc.old_occurrences_section {
      if bidi_links && !has_missing_links_with_path(issues, &doc.relative_path) {
        issues.push(Issue::ObsoleteOccurrencesSection {
          location: Location {
            file: doc.relative_path.clone(),
            line: old_occurrences_section.line_number,
            start: old_occurrences_section.title_text_start as u32,
            end: old_occurrences_section.title_text_end(),
          },
        });
      }
    }
  }
}

/// indicates whether the given issue list contains a `MissingLink` issue with the given path
fn has_missing_links_with_path(issues: &[Issue], path: &str) -> bool {
  issues
    .iter()
    .any(|issue| is_missing_link_with_path(issue, path))
}

/// indicates whether the given issue is a `MissingLink` issue with the given path
fn is_missing_link_with_path(issue: &Issue, path: &str) -> bool {
  if let Issue::MissingLink {
    location,
    path: _,
    title: _,
  } = issue
  {
    location.file == path
  } else {
    false
  }
}

#[cfg(test)]
mod tests {

  mod is_missing_link_with_path {
    use crate::check::{Issue, Location};
    use big_s::S;

    #[test]
    fn matching() {
      let location = Location {
        file: S("file.md"),
        ..Location::default()
      };
      let issue = Issue::MissingLink {
        location,
        path: S("missing.md"),
        title: S("title"),
      };
      let have = super::super::is_missing_link_with_path(&issue, "file.md");
      let want = true;
      assert_eq!(have, want);
    }

    #[test]
    fn mismatching_filename() {
      let location = Location {
        file: S("file.md"),
        ..Location::default()
      };
      let issue = Issue::MissingLink {
        location,
        path: S("missing.md"),
        title: S("title"),
      };
      let have = super::super::is_missing_link_with_path(&issue, "other.md");
      let want = false;
      assert_eq!(have, want);
    }

    #[test]
    fn mismatching_enum_variant() {
      let location = Location {
        file: S("file.md"),
        ..Location::default()
      };
      let issue = Issue::BrokenImage {
        location,
        target: S("foo.png"),
      };
      let have = super::super::is_missing_link_with_path(&issue, "other.md");
      let want = false;
      assert_eq!(have, want);
    }
  }
}
