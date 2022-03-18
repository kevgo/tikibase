use crate::database::{section, Tikibase};
use crate::issue::MissingLink;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use std::borrow::Cow;
use std::path::Path;

pub fn add_missing_links(base: &mut Tikibase, file: &Path, links: &Vec<MissingLink>) -> String {
    let doc = base.get_doc_mut(file).unwrap();

    // append a newline to the section before
    doc.last_section_mut().push_line("");

    // insert occurrences section
    let mut section_builder = section::Builder::new("### occurrences", doc.lines_count() + 1);
    section_builder.add_line("");
    for link in links {
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
    doc.save(&base.dir);
    result
}

/// removes all links from the given string
fn strip_links(text: &str) -> Cow<str> {
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
static SOURCE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"\[([^]]*)\]\([^)]*\)"#).unwrap());

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
