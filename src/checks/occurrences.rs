use super::doc_links::DocLinks;
use super::Issue;
use super::Issues;
use crate::config;
use crate::database::document::builder_with_title_line;
use crate::database::tikibase::Tikibase;
use ahash::AHashSet;
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::borrow::Cow;
use std::path::PathBuf;

struct MissingLink {
    path: PathBuf,
    title: String,
}

/// missing links in a document
pub struct MissingLinks {
    file: PathBuf,
    links: Vec<MissingLink>,
}

impl Issue for MissingLinks {
    fn fix(&self, base: &mut Tikibase, _config: &config::Data) -> String {
        let base_dir = base.dir.clone();
        let doc = base.get_doc_mut(&self.file).unwrap();

        // append a newline to the section before
        doc.last_section_mut().push_line("");

        // insert occurrences section
        let mut section_builder = builder_with_title_line("### occurrences", doc.lines_count() + 1);
        section_builder.add_body_line("");
        for link in &self.links {
            section_builder.add_body_line(format!(
                "- [{}]({})",
                strip_links(&link.title),
                link.path.to_string_lossy()
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
        let links: Vec<Cow<str>> = self
            .links
            .iter()
            .map(|ml| ml.path.to_string_lossy())
            .collect();
        format!(
            "{}  missing link to {}",
            self.file.to_string_lossy(),
            links.join(", "),
        )
    }
}

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

/// removes all links from the given string
fn strip_links(text: &str) -> Cow<str> {
    lazy_static! {
        static ref SOURCE_RE: Regex = Regex::new(r#"\[([^]]*)\]\([^)]*\)"#).unwrap();
    }
    let matches: Vec<Captures> = SOURCE_RE.captures_iter(text).collect();
    if matches.is_empty() {
        return Cow::Borrowed(text);
    }
    let mut result = text.to_string();
    for m in matches {
        result = result.replace(m.get(0).unwrap().as_str(), m.get(1).unwrap().as_str());
    }
    Cow::Owned(result)
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
        issues.push(Box::new(MissingLinks {
            file: doc.path.clone(),
            links: missing_outgoing
                .into_iter()
                .map(|path| base.get_doc(&path).unwrap())
                .map(|doc| MissingLink {
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
    use crate::database::tikibase::Tikibase;
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

    mod strip_links {

        #[test]
        fn with_links() {
            let have = super::super::strip_links("[one](1.md) [two](2.md)");
            assert!(have.is_owned());
            assert_eq!(have, "one two");
        }

        #[test]
        fn without_links() {
            let have = super::super::strip_links("one two");
            assert!(have.is_borrowed());
            assert_eq!(have, "one two");
        }
    }
}
