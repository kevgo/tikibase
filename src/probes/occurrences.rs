use crate::database::DocLinks;
use crate::issue::Issue;
use crate::issue::MissingLink;
use crate::Tikibase;
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
            // no missing links --> done here
            if let Some(occurrences_section_line) = doc.occurrences_section_line {
                issues.push(Issue::ObsoleteLink {
                    file: doc.path.clone(),
                    line: occurrences_section_line,
                });
            }
            continue;
        }

        // register missing occurrences
        missing_outgoing.sort();
        issues.push(Issue::MissingLinks {
            file: doc.path.clone(),
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

    use crate::database::DocLinks;
    use crate::testhelpers::{create_file, empty_config, tmp_dir};
    use crate::Tikibase;

    #[test]
    fn process() {
        let dir = tmp_dir();
        create_file("1.md", "# One\n", &dir);
        create_file("2.md", "# Two\n\n[one](1.md)\n", &dir);
        create_file("3.md", "# Three\n\n[one](1.md)\n", &dir);
        let (base, errs) = Tikibase::load(dir, &empty_config());
        assert_eq!(errs.len(), 0);
        let mut outgoing_links = DocLinks::default();
        outgoing_links.add("3.md", "1.md");
        outgoing_links.add("2.md", "1.md");
        let mut incoming_links = DocLinks::default();
        incoming_links.add("1.md", "3.md");
        incoming_links.add("1.md", "2.md");
        let have = super::scan(&base, &incoming_links, &outgoing_links);
        let issues: Vec<String> = have.iter().map(|issue| issue.to_string()).collect();
        assert_eq!(issues, vec!["1.md  missing link to 2.md, 3.md"]);
    }
}
