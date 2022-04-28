use super::{check_doc_2, Issue, Location, State2};
use crate::database::Directory;
use std::path::{Path, PathBuf};

// phase 2 `Directory` check
pub(crate) fn check_dir_2(dir: &Directory, state: &mut State2) {
    for (name, doc) in &dir.docs {
        let doc_path = dir.relative_path.join(name);
        check_doc_2(doc, state);
        if let Some(bidi_links) = dir.config.bidi_links {
            if let Some(old_occurrences_section) = &doc.old_occurrences_section {
                if bidi_links && !has_missing_link_with_path(&state.issues, &doc_path) {
                    state.issues.push(Issue::ObsoleteOccurrencesSection {
                        location: Location {
                            file: doc_path,
                            line: old_occurrences_section.line_number,
                            start: old_occurrences_section.title_text_start as u32,
                            end: old_occurrences_section.title_text_end(),
                        },
                    });
                }
            }
        }
    }
    for resource in dir.resources.keys() {
        let full_path = dir.relative_path.join(resource);
        if !state.linked_resources.contains(&full_path) {
            state.issues.push(Issue::OrphanedResource {
                location: Location {
                    file: PathBuf::from(resource),
                    line: 0,
                    start: 0,
                    end: 0,
                },
            });
        }
    }
    for dir in dir.dirs.values() {
        check_dir_2(dir, state);
    }
}

/// indicates whether the given issue list contains a `MissingLink` issue with the given path
fn has_missing_link_with_path(issues: &[Issue], path: &Path) -> bool {
    issues
        .iter()
        .any(|issue| is_missing_link_with_path(issue, path))
}

/// indicates whether the given issue is a `MissingLink` issue with the given path
fn is_missing_link_with_path(issue: &Issue, path: &Path) -> bool {
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
        use std::path::PathBuf;

        use crate::check::{Issue, Location};

        #[test]
        fn matching() {
            let location = Location {
                file: PathBuf::from("file.md"),
                ..Location::default()
            };
            let issue = Issue::MissingLink {
                location,
                path: PathBuf::from("missing.md"),
                title: "title".into(),
            };
            let have = super::super::is_missing_link_with_path(&issue, &PathBuf::from("file.md"));
            let want = true;
            assert_eq!(have, want);
        }

        #[test]
        fn mismatching_filename() {
            let location = Location {
                file: PathBuf::from("file.md"),
                ..Location::default()
            };
            let issue = Issue::MissingLink {
                location,
                path: PathBuf::from("missing.md"),
                title: "title".into(),
            };
            let have = super::super::is_missing_link_with_path(&issue, &PathBuf::from("other.md"));
            let want = false;
            assert_eq!(have, want);
        }

        #[test]
        fn mismatching_enum_variant() {
            let location = Location {
                file: PathBuf::from("file.md"),
                ..Location::default()
            };
            let issue = Issue::BrokenImage {
                location,
                target: "foo.png".into(),
            };
            let have = super::super::is_missing_link_with_path(&issue, &PathBuf::from("other.md"));
            let want = false;
            assert_eq!(have, want);
        }
    }
}
