use super::doc_links::DocLinks;
use super::Issue;
use super::Issues;
use crate::checks::issues;
use crate::config;
use crate::database::Tikibase;
use ahash::AHashSet;
use std::path::PathBuf;

/// indicates that a document contains an "occurrences" section
/// that should no longer be there
pub struct ObsoleteLink {
    file: PathBuf,
    line: u32,
}

impl Issue for ObsoleteLink {
    fn describe(&self) -> String {
        format!(
            "{}:{}  obsolete occurrences section",
            self.file.to_string_lossy(),
            self.line + 1,
        )
    }

    fn fix(&self, base: &mut Tikibase, _config: &config::Data) -> String {
        let base_dir = base.dir.clone();
        let doc = base.get_doc_mut(&self.file).unwrap();
        // we can simply flush the document here because
        // its "occurrences" section was filtered out when loading the document
        doc.flush(&base_dir);
        format!(
            "{}:{}  removed obsolete occurrences section",
            self.file.to_string_lossy(),
            self.line + 1,
        )
    }

    fn fixable(&self) -> bool {
        true
    }
}

pub fn process(
    base: &Tikibase,
    incoming_doc_links: &DocLinks,
    outgoing_doc_links: &DocLinks,
) -> Issues {
    let mut issues = Issues::new();
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
                issues.push(Box::new(ObsoleteLink {
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

    use crate::checks::doc_links::DocLinks;
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
