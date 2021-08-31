use crate::database::DocLinks;
use crate::database::Tikibase;
use crate::issues;
use crate::Issue;
use ahash::AHashSet;
use std::path::PathBuf;

pub fn process(
    base: &Tikibase,
    incoming_doc_links: &DocLinks,
    outgoing_doc_links: &DocLinks,
) -> Vec<Box<dyn Issue>> {
    let mut issues = Vec::<Box<dyn Issue>>::new();
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

        // no missing links --> done here
        if missing_outgoing.is_empty() {
            if let Some(occurrences_section_line) = doc.occurrences_section_line {
                issues.push(Box::new(issues::ObsoleteLink {
                    file: doc.path.clone(),
                    line: occurrences_section_line,
                }));
            }
            continue;
        }

        // register missing occurrences
        missing_outgoing.sort();
        issues.push(Box::new(issues::MissingLinks {
            file: doc.path.clone(),
            links: missing_outgoing
                .into_iter()
                .map(|path| base.get_doc(&path).unwrap())
                .map(|doc| issues::MissingLink {
                    path: doc.path.clone(),
                    title: doc.title().into(),
                })
                .collect(),
        }));
    }
    issues
}

#[cfg(test)]
mod tests {

    use crate::database::DocLinks;
    use crate::database::Tikibase;
    use crate::testhelpers::{create_file, empty_config, tmp_dir};

    #[test]
    fn process() {
        let dir = tmp_dir();
        create_file("1.md", "# One\n", &dir);
        create_file("2.md", "# Two\n\n[one](1.md)\n", &dir);
        create_file("3.md", "# Three\n\n[one](1.md)\n", &dir);
        let (base, errs) = Tikibase::load(dir, &empty_config());
        assert_eq!(errs.len(), 0);
        let mut outgoing_links = DocLinks::new();
        outgoing_links.add("3.md", "1.md");
        outgoing_links.add("2.md", "1.md");
        let mut incoming_links = DocLinks::new();
        incoming_links.add("1.md", "3.md");
        incoming_links.add("1.md", "2.md");
        let have = super::process(&base, &incoming_links, &outgoing_links);
        let issues: Vec<String> = have.iter().map(|issue| issue.describe()).collect();
        assert_eq!(issues, vec!["1.md  missing link to 2.md, 3.md"]);
    }
}
