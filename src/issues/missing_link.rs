use crate::config;
use crate::database::section;
use crate::database::Tikibase;
use crate::Issue;
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::borrow::Cow;
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;

pub struct MissingLink {
    pub path: PathBuf,
    pub title: String,
}

/// missing links in a document
pub struct MissingLinks {
    pub file: PathBuf,
    pub links: Vec<MissingLink>,
}

impl Issue for MissingLinks {
    fn fix(&self, base: &mut Tikibase, _config: &config::Data) -> String {
        let base_dir = base.dir.clone();
        let doc = base.get_doc_mut(&self.file).unwrap();

        // append a newline to the section before
        doc.last_section_mut().push_line("");

        // insert occurrences section
        let mut section_builder = section::Builder::new("### occurrences", doc.lines_count() + 1);
        section_builder.add_line("");
        for link in &self.links {
            section_builder.add_line(format!(
                "- [{}]({})",
                strip_links(&link.title),
                link.path.to_string_lossy()
            ));
        }
        let occurrences_section = section_builder.result();
        let result = format!(
            "{}:{}  added occurrences section",
            doc.path.to_string_lossy(),
            occurrences_section.line_number + 1
        );
        doc.content_sections.push(occurrences_section);
        doc.save(&base_dir);
        result
    }

    fn fixable(&self) -> bool {
        true
    }
}

impl Display for MissingLinks {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let links: Vec<Cow<str>> = self
            .links
            .iter()
            .map(|ml| ml.path.to_string_lossy())
            .collect();
        write!(
            f,
            "{}  missing link to {}",
            self.file.to_string_lossy(),
            links.join(", "),
        )
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

#[cfg(test)]
mod tests {

    mod strip_links {

        #[test]
        fn with_links() {
            let have = super::super::strip_links("[one](1.md) [two](2.md)");
            assert_eq!(have, "one two");
        }

        #[test]
        fn without_links() {
            let have = super::super::strip_links("one two");
            assert_eq!(have, "one two");
        }
    }
}
