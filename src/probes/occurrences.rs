use super::link_broken::DocLinks;
use super::outcome::Outcome;
use crate::core::document::builder_with_title_line;
use crate::core::tikibase::Tikibase;
use std::cmp::{Eq, Ord, Ordering, PartialEq};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

#[derive(Eq)]
struct MissingOccurrence {
    path: PathBuf,
    title: String,
}

impl Ord for MissingOccurrence {
    fn cmp(&self, other: &Self) -> Ordering {
        self.path.cmp(&other.path)
    }
}

impl PartialOrd for MissingOccurrence {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.path.cmp(&other.path))
    }
}

impl PartialEq for MissingOccurrence {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

pub fn process(mut base: Tikibase, doc_links: DocLinks, fix: bool) -> Outcome {
    let mut result = Outcome::new();
    let mut missings = HashMap::<PathBuf, Vec<MissingOccurrence>>::new();
    for doc in &base.docs {
        // determine outgoing links
        let outgoing: HashSet<&PathBuf> = doc_links
            .iter()
            .filter(|doclink| doclink.from == doc.path)
            .map(|doclink| &doclink.to)
            .collect();

        // determine incoming links
        let incoming: HashSet<&PathBuf> = doc_links
            .iter()
            .filter(|doclink| doclink.to == doc.path)
            .map(|doclink| &doclink.from)
            .collect();

        // determine missing links in this document
        let missing_outgoing: HashSet<&PathBuf> = incoming.difference(&outgoing).copied().collect();
        let mut m: Vec<&PathBuf> = missing_outgoing.iter().copied().collect();

        // no missing links --> done here
        if m.is_empty() {
            continue;
        }

        // register missing occurrences
        m.sort();
        missings.insert(
            doc.path.clone(),
            missing_outgoing
                .into_iter()
                .map(|path| base.get_doc(path).unwrap())
                .map(|doc| MissingOccurrence {
                    path: doc.path.clone(),
                    title: doc.title(),
                })
                .collect(),
        );
    }

    if fix {
        let base_dir = base.dir.clone();
        for (filepath, mut missing_occurrences) in missings {
            missing_occurrences.sort();
            let doc = base.get_doc_mut(&filepath).unwrap();
            // insert a newline into the section before
            let last_section = doc.last_section_mut();
            last_section.push_line("");

            // insert occurrences section
            let mut section_builder =
                builder_with_title_line("### occurrences".to_string(), doc.lines_count() + 1);
            section_builder.add_body_line("".to_string());
            for missing_occurrence in missing_occurrences {
                section_builder.add_body_line(format!(
                    "- [{}]({})",
                    missing_occurrence.title,
                    &missing_occurrence.path.to_string_lossy()
                ));
            }
            let occurrences_section = section_builder.result().unwrap();
            result.fixes.push(format!(
                "{}:{}  added occurrences section",
                doc.path.to_string_lossy(),
                occurrences_section.line_number + 1
            ));
            doc.content_sections.push(occurrences_section);
            doc.flush(&base_dir);
        }
    } else {
        for (filepath, mut missing_occurrences) in missings {
            missing_occurrences.sort();
            for missing_occurrence in missing_occurrences {
                result.findings.push(format!(
                    "{}  missing link to \"{}\"",
                    filepath.to_string_lossy(),
                    missing_occurrence.title,
                ));
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {

    use std::path::PathBuf;

    use crate::core::tikibase::Tikibase;
    use crate::probes::link_broken::DocLinks;
    use crate::{probes::link_broken::DocLink, testhelpers};

    #[test]
    fn process_false() {
        let dir = testhelpers::tmp_dir();
        testhelpers::create_file("1.md", "# One\n", &dir);
        testhelpers::create_file("2.md", "# Two\n\n[one](1.md)\n", &dir);
        testhelpers::create_file("3.md", "# Three\n\n[one](1.md)\n", &dir);
        let (base, errs) = Tikibase::load(dir);
        assert_eq!(errs.len(), 0);
        let mut doc_links: DocLinks = Vec::new();
        doc_links.push(DocLink {
            from: PathBuf::from("3.md"),
            to: PathBuf::from("1.md"),
        });
        doc_links.push(DocLink {
            from: PathBuf::from("2.md"),
            to: PathBuf::from("1.md"),
        });
        let have = super::process(base, doc_links, false);
        assert_eq!(have.fixes.len(), 0);
        assert_eq!(
            have.findings,
            vec![
                "1.md  missing link to \"Two\"",
                "1.md  missing link to \"Three\"",
            ]
        );
    }

    #[test]
    fn process_true() {
        let dir = testhelpers::tmp_dir();
        testhelpers::create_file("1.md", "# One\n", &dir);
        testhelpers::create_file("2.md", "# Two\n\n[one](1.md)\n", &dir);
        testhelpers::create_file("3.md", "# Three\n\n[one](1.md)\n", &dir);
        let (base, errs) = Tikibase::load(dir.clone());
        assert_eq!(errs.len(), 0);
        let mut doc_links: DocLinks = Vec::new();
        doc_links.push(DocLink {
            from: PathBuf::from("3.md"),
            to: PathBuf::from("1.md"),
        });
        doc_links.push(DocLink {
            from: PathBuf::from("2.md"),
            to: PathBuf::from("1.md"),
        });
        let have = super::process(base, doc_links, true);
        assert_eq!(have.fixes, vec!["1.md:3  added occurrences section"]);
        assert_eq!(have.findings.len(), 0);
        let content_one = testhelpers::load_file("1.md", &dir);
        assert_eq!(
            content_one,
            "# One\n\n### occurrences\n\n- [Two](2.md)\n- [Three](3.md)\n"
        )
    }
}
