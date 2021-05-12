use super::doc_links::DocLinks;
use super::Issue;
use super::Issues;
use crate::core::document::builder_with_title_line;
use crate::core::tikibase::Tikibase;
use std::path::PathBuf;

struct MissingOccurrence {
    path: PathBuf,
    title: String,
}

/// missing links in a document
pub struct MissingOccurrences {
    file: PathBuf,
    missing_links: Vec<MissingOccurrence>,
}

impl Issue for MissingOccurrences {
    fn fix(&self, base: &mut Tikibase) -> String {
        let base_dir = base.dir.clone();
        let doc = base.get_doc_mut(&self.file).unwrap();

        // append a newline to the section before
        doc.last_section_mut().push_line("");

        // insert occurrences section
        let mut section_builder =
            builder_with_title_line("### occurrences".to_string(), doc.lines_count() + 1);
        section_builder.add_body_line("".to_string());
        for missing in self.missing_links.iter() {
            section_builder.add_body_line(format!(
                "- [{}]({})",
                missing.title,
                missing.path.to_string_lossy()
            ));
        }
        let occurrences_section = section_builder.result().unwrap();
        let result = format!(
            "{}:{}  added occurrences section",
            doc.path.to_string_lossy(),
            occurrences_section.line_number + 1
        );
        doc.content_sections.push(occurrences_section);
        doc.flush(&base_dir);
        result
    }

    fn fixable(&self) -> bool {
        true
    }

    fn describe(&self) -> String {
        let links: Vec<String> = self
            .missing_links
            .iter()
            .map(|occ| occ.path.to_string_lossy().to_string())
            .collect();
        format!(
            "{}  missing link to {}",
            self.file.to_string_lossy(),
            links.join(", "),
        )
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
            .difference(&outgoing_doc_links.get(&doc.path))
            .into_iter()
            // TODO: use reference here instead of cloning
            .map(|p| p.to_owned())
            .collect();

        // no missing links --> done here
        if missing_outgoing.is_empty() {
            continue;
        }

        // register missing occurrences
        missing_outgoing.sort();
        issues.push(Box::new(MissingOccurrences {
            file: doc.path.clone(),
            missing_links: missing_outgoing
                .into_iter()
                .map(|path| base.get_doc(&path).unwrap())
                .map(|doc| MissingOccurrence {
                    path: doc.path.clone(),
                    title: doc.title(),
                })
                .collect(),
        }));
    }
    issues
}

#[cfg(test)]
mod tests {

    use crate::core::tikibase::Tikibase;
    use crate::probes::doc_links::DocLinks;
    use crate::testhelpers;
    use std::path::PathBuf;

    #[test]
    fn process() {
        let dir = testhelpers::tmp_dir();
        testhelpers::create_file("1.md", "# One\n", &dir);
        testhelpers::create_file("2.md", "# Two\n\n[one](1.md)\n", &dir);
        testhelpers::create_file("3.md", "# Three\n\n[one](1.md)\n", &dir);
        let (base, errs) = Tikibase::load(dir);
        assert_eq!(errs.len(), 0);
        let mut outgoing_links = DocLinks::new();
        outgoing_links.add(PathBuf::from("3.md"), PathBuf::from("1.md"));
        outgoing_links.add(PathBuf::from("2.md"), PathBuf::from("1.md"));
        let mut incoming_links = DocLinks::new();
        incoming_links.add(PathBuf::from("1.md"), PathBuf::from("3.md"));
        incoming_links.add(PathBuf::from("1.md"), PathBuf::from("2.md"));
        let have = super::process(&base, &incoming_links, &outgoing_links);
        let issues: Vec<String> = have.iter().map(|issue| issue.describe()).collect();
        assert_eq!(issues, vec!["1.md  missing link to 2.md, 3.md",]);
    }
}
