use super::Fix;
use crate::commands::MissingLink;
use crate::database::{section, Tikibase};
use crate::Location;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use std::borrow::Cow;

pub fn add_occurrences(base: &mut Tikibase, location: Location, links: Vec<MissingLink>) -> Fix {
    let base_dir = base.dir.clone();
    let doc = base.get_doc_mut(&location.file).unwrap();

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
    let line = occurrences_section.line_number;
    doc.content_sections.push(occurrences_section);
    doc.save(&base_dir);
    Fix::AddedOccurrencesSection {
        location: Location {
            file: location.file,
            line,
        },
    }
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
        use super::super::strip_links;

        #[test]
        fn with_links() {
            let have = strip_links("[one](1.md) [two](2.md)");
            assert_eq!(have, "one two");
        }

        #[test]
        fn without_links() {
            let have = strip_links("one two");
            assert_eq!(have, "one two");
        }
    }
}
