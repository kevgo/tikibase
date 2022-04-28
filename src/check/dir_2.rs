use super::scanners::{section_capitalization, section_level};
use super::{check_doc_2, Issue, Location};
use crate::database::Directory;
use ahash::AHashMap;
use std::path::PathBuf;

/// check phase 2
pub(crate) fn check_dir_2(
    dir: &Directory,
    linked_resources: &[PathBuf],
    issues: &mut Vec<Issue>,
    cap_outliers: &AHashMap<String, section_capitalization::OutlierInfo>,
    level_outliers: &AHashMap<String, section_level::OutlierInfo>,
) {
    for (name, doc) in &dir.docs {
        let doc_path = dir.relative_path.join(name);
        check_doc_2(doc, issues, cap_outliers, level_outliers);
        if let Some(bidi_links) = dir.config.bidi_links {
            if let Some(old_occurrences_section) = &doc.old_occurrences_section {
                if bidi_links
                    && !issues.iter().any(|issue| {
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
                    issues.push(Issue::ObsoleteOccurrencesSection {
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
        if !linked_resources.contains(&full_path) {
            issues.push(Issue::OrphanedResource {
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
        check_dir_2(dir, linked_resources, issues, cap_outliers, level_outliers);
    }
}
