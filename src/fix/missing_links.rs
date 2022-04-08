use super::{Fix, FixResult};
use crate::commands::MissingLink;
use crate::database::{section, Tikibase};
use crate::{Config, Issue, Location};
use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use std::borrow::Cow;

pub fn add_occurrences(
    base: &mut Tikibase,
    location: Location,
    links: Vec<MissingLink>,
    config: &Config,
) -> FixResult {
    let base_dir = base.dir.clone();
    let doc = base.get_doc_mut(&location.file).unwrap();

    // append a newline to the section before
    doc.last_section_mut().push_line("");

    // insert occurrences section
    let mut section_builder = section::Builder::new("### occurrences", doc.lines_count() + 1);
    section_builder.add_line("");
    let regex = match config.title_regex() {
        Ok(regex) => regex,
        Err(issue) => return FixResult::Failed(issue),
    };
    for link in links {
        let stripped_title = &strip_links(&link.title);
        let title = match &regex {
            Some(regex) => match extract_shortcut(stripped_title, regex) {
                ExtractShortcutResult::ShortcutFound(shortcut) => shortcut,
                ExtractShortcutResult::Failed(issue) => return FixResult::Failed(issue),
                ExtractShortcutResult::NoShortcutFound => stripped_title,
            },
            None => stripped_title,
        };
        section_builder.add_line(format!("- [{}]({})", title, link.path.to_string_lossy()));
    }
    let occurrences_section = section_builder.result();
    let line = occurrences_section.line_number;
    let end = occurrences_section.title_line.text.len() as u32;
    doc.content_sections.push(occurrences_section);
    doc.save(&base_dir);
    FixResult::Fixed(Fix::AddedOccurrencesSection {
        location: Location {
            file: location.file,
            line,
            start: 0,
            end,
        },
    })
}

fn extract_shortcut<'a>(title: &'a str, regex: &Regex) -> ExtractShortcutResult<'a> {
    match regex.captures_len() {
        0 | 1 => {
            return ExtractShortcutResult::Failed(Issue::TitleRegexNoCaptures {
                regex: regex.to_string(),
            })
        }
        2 => {}
        other => {
            return ExtractShortcutResult::Failed(Issue::TitleRegexTooManyCaptures {
                regex: regex.to_string(),
                captures: other - 1,
            })
        }
    }
    match regex.captures(title) {
        Some(captures) => match captures.len() {
            2 => ExtractShortcutResult::ShortcutFound(captures.get(1).unwrap().as_str()),
            other => ExtractShortcutResult::Failed(Issue::TitleRegexTooManyCaptures {
                regex: regex.to_string(),
                captures: other - 1,
            }),
        },
        None => ExtractShortcutResult::NoShortcutFound,
    }
}

#[derive(Debug, PartialEq)]
enum ExtractShortcutResult<'a> {
    /// found a shortcut
    ShortcutFound(&'a str),
    /// the regex worked but the given title doesn't define a shortcut
    NoShortcutFound,
    /// a problem with the given Regex occurred
    Failed(Issue),
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

    mod extract_shortcut {
        use crate::fix::missing_links::{extract_shortcut, ExtractShortcutResult};
        use crate::Issue;
        use regex::Regex;

        #[test]
        fn works() {
            let regex = Regex::new("\\((\\w+)\\)").unwrap();
            let give = "# Example Title (ET)";
            let want = ExtractShortcutResult::ShortcutFound("ET");
            let have = extract_shortcut(give, &regex);
            assert_eq!(have, want);
        }

        #[test]
        fn title_without_shortcut() {
            let regex = Regex::new("\\((\\w+)\\)").unwrap();
            let give = "# Example Title";
            let want = ExtractShortcutResult::NoShortcutFound;
            let have = extract_shortcut(give, &regex);
            assert_eq!(have, want);
        }

        #[test]
        fn regex_without_capture() {
            let regex = Regex::new("123").unwrap();
            let give = "# Example Title (ET)";
            let want = ExtractShortcutResult::Failed(Issue::TitleRegexNoCaptures {
                regex: "123".into(),
            });
            let have = extract_shortcut(give, &regex);
            assert_eq!(have, want);
        }

        #[test]
        fn regex_two_captures() {
            let regex = Regex::new("(\\w) (\\w)").unwrap();
            let give = "# Example Title";
            let want = ExtractShortcutResult::Failed(Issue::TitleRegexTooManyCaptures {
                regex: "(\\w) (\\w)".into(),
                captures: 2,
            });
            let have = extract_shortcut(give, &regex);
            assert_eq!(have, want);
        }
    }

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
