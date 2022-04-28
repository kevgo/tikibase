use crate::commands::MissingLink;
use crate::{Issue, Location, Tikibase};
use ahash::AHashSet;
use std::path::PathBuf;

pub(crate) fn scan(base: &Tikibase) -> Vec<Issue> {
    let mut issues = Vec::new();
    for (_path, doc) in &base.dir.docs {
        let mut missing_outgoing: Vec<PathBuf> = incoming_doc_links
            .get(&doc.relative_path)
            .get_or_insert(&AHashSet::new())
            .difference(
                outgoing_doc_links
                    .get(&doc.relative_path)
                    .get_or_insert(&AHashSet::new()),
            )
            .into_iter()
            .cloned()
            .collect();

        if missing_outgoing.is_empty() {
            // no missing links --> done with this document
            if let Some(old_occurrences_section) = doc.old_occurrences_section.as_ref() {
                issues.push(Issue::ObsoleteOccurrencesSection {
                    location: Location {
                        file: doc.relative_path.clone(),
                        line: old_occurrences_section.line_number,
                        start: old_occurrences_section.title_text_start as u32,
                        end: old_occurrences_section.title_text_end(),
                    },
                });
            }
            continue;
        }

        // register missing occurrences
        missing_outgoing.sort();
        issues.push(Issue::MissingLinks {
            location: Location {
                file: doc.relative_path.clone(),
                line: doc.lines_count(),
                start: 0,
                end: doc.last_line().text.len() as u32,
            },
            links: missing_outgoing
                .into_iter()
                .map(|path| base.get_doc(&path).unwrap())
                .map(|doc| MissingLink {
                    path: doc.relative_path.clone(),
                    title: doc.human_title().into(),
                })
                .collect(),
        });
    }
    issues
}
