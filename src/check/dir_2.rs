use super::{check_doc_2, Issue, Location, State2};
use crate::database::Directory;
use std::path::PathBuf;

// phase 2 `Directory` check
pub(crate) fn check_dir_2(dir: &Directory, state: &mut State2) {
    for (doc_name, doc) in &dir.docs {
        let doc_path = dir.relative_path.join(doc_name);
        check_doc_2(doc, state);
        if let Some(bidi_links) = dir.config.bidi_links {
            if let Some(old_occurrences_section) = &doc.old_occurrences_section {
                if bidi_links
                    && !state.issues.iter().any(|issue| {
                        if let Issue::MissingLink {
                            location,
                            path: _,
                            title: _,
                        } = issue
                        {
                            location.file == doc_path
                        } else {
                            false
                        }
                    })
                {
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
