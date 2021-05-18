use super::doc_links::DocLinks;
use super::Issue;
use super::Issues;
use crate::config;
use crate::core::document::builder_with_title_line;
use crate::core::tikibase::Tikibase;
use ahash::AHashSet;
use lazy_static::lazy_static;
use regex::{Captures, Regex};
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
    fn fix(&self, base: &mut Tikibase, _config: &config::Data) -> String {
        let base_dir = base.dir.clone();
        let doc = base.get_doc_mut(&self.file).unwrap();

        // append a newline to the section before
        doc.last_section_mut().push_line("");

        // insert occurrences section
        let mut section_builder = builder_with_title_line("### occurrences", doc.lines_count() + 1);
        section_builder.add_body_line("");
        for missing in self.missing_links.iter() {
            section_builder.add_body_line(format!(
                "- [{}]({})",
                strip_links(&missing.title),
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

/// removes all links from the given string
// TODO: try using a Cow<> here
fn strip_links(text: &str) -> String {
    lazy_static! {
        static ref SOURCE_RE: Regex = Regex::new(r#"\[([^]]*)\]\([^)]*\)"#).unwrap();
    }
    let matches: Vec<Captures> = SOURCE_RE.captures_iter(text).collect();
    if matches.is_empty() {
        return text.to_string();
    }
    let mut result = text.to_string();
    for m in matches {
        result = result.replace(m.get(0).unwrap().as_str(), m.get(1).unwrap().as_str());
    }
    result
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
                &outgoing_doc_links
                    .get(&doc.path)
                    .get_or_insert(&AHashSet::new()),
            )
            .into_iter()
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

    #[test]
    fn strip_links() {
        let tests = vec![
            ("[one](1.md) [two](2.md)", "one two"),
            ("one two", "one two"),
        ];
        for (give, want) in tests {
            let have = super::strip_links(give);
            assert_eq!(have, want, "{} -> {}", give, want);
        }
    }
}
