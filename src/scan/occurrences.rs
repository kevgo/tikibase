use crate::commands::MissingLink;
use crate::database::DocLinks;
use crate::{Issue, Location, Tikibase};
use ahash::AHashSet;
use std::path::PathBuf;

pub(crate) fn scan(
    base: &Tikibase,
    incoming_doc_links: &DocLinks,
    outgoing_doc_links: &DocLinks,
) -> Vec<Issue> {
    let mut issues = Vec::new();
    for doc in &base.docs {
        let mut missing_outgoing: Vec<PathBuf> = incoming_doc_links
            .get(&doc.path)
            .get_or_insert(&AHashSet::new())
            .difference(
                outgoing_doc_links
                    .get(&doc.path)
                    .get_or_insert(&AHashSet::new()),
            )
            .into_iter()
            .cloned()
            .collect();

        if missing_outgoing.is_empty() {
            // no missing links --> done with this document
            if let Some(occurrences_section_line) = doc.occurrences_section_line {
                issues.push(Issue::ObsoleteOccurrencesSection {
                    location: Location {
                        file: doc.path.clone(),
                        line: occurrences_section_line,
                    },
                });
            }
            continue;
        }

        // register missing occurrences
        missing_outgoing.sort();
        issues.push(Issue::MissingLinks {
            location: Location {
                file: doc.path.clone(),
                line: doc.lines_count(),
            },
            links: missing_outgoing
                .into_iter()
                .map(|path| base.get_doc(&path).unwrap())
                .map(|doc| MissingLink {
                    path: doc.path.clone(),
                    title: doc.title().into(),
                })
                .collect(),
        });
    }
    issues
}

#[cfg(test)]
mod tests {
    use crate::commands::MissingLink;
    use crate::database::DocLinks;
    use crate::{test, Config, Issue, Location, Tikibase};

    #[test]
    fn process() {
        let dir = test::tmp_dir();
        test::create_file("1.md", "# One\n", &dir);
        test::create_file("2.md", "# Two\n\n[one](1.md)\n", &dir);
        test::create_file("3.md", "# Three\n\n[one](1.md)\n", &dir);
        let base = Tikibase::load(dir, &Config::default()).unwrap();
        let mut outgoing_links = DocLinks::default();
        outgoing_links.add("3.md", "1.md");
        outgoing_links.add("2.md", "1.md");
        let mut incoming_links = DocLinks::default();
        incoming_links.add("1.md", "3.md");
        incoming_links.add("1.md", "2.md");
        let have = super::scan(&base, &incoming_links, &outgoing_links);
        pretty::assert_eq!(
            have,
            vec![Issue::MissingLinks {
                location: Location {
                    file: "1.md".into(),
                    line: 0
                },
                links: vec![
                    MissingLink {
                        path: "2.md".into(),
                        title: "Two".into()
                    },
                    MissingLink {
                        path: "3.md".into(),
                        title: "Three".into()
                    }
                ]
            }]
        );
    }
}
