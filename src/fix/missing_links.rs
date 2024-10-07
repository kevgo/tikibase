use super::Fix::AddedOccurrencesSection;
use crate::check::Issue::{self, TitleRegexNoCaptures, TitleRegexTooManyCaptures};
use crate::check::Location;
use crate::database::{section, Line, Tikibase};
use crate::fix;
use crate::fix::Result::{Failed, Fixed};
use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use section::Section;
use std::borrow::Cow;

pub fn add_occurrences(
  base: &mut Tikibase,
  location: Location,
  path: String,
  title: &str,
) -> fix::Result {
  let base_dir = base.root.clone();
  let title_regex = match base.dir.config.title_regex() {
    Ok(regex) => regex,
    Err(issue) => return Failed(issue),
  };
  let doc = base.get_doc_mut(&location.file).unwrap();

  // check for existing occurrences section
  let occurrences_section = if let Some(section) = doc.section_with_human_title_mut("occurrences") {
    section
  } else {
    // append a newline to the section before
    doc.last_section_mut().push_line("");

    // insert occurrences section
    let occurrences_section = Section::new(doc.lines_count() + 1, "### occurrences", vec![""]);
    doc.content_sections.push(occurrences_section);
    doc.section_with_human_title_mut("occurrences").unwrap()
  };
  let stripped_title = &strip_links(title);
  let title = match &title_regex {
    None => stripped_title,
    Some(regex) => match extract_shortcut(stripped_title, regex) {
      ExtractShortcutResult::ShortcutFound(shortcut) => shortcut,
      ExtractShortcutResult::NoShortcutFound => stripped_title,
      ExtractShortcutResult::Failed(issue) => return Failed(issue),
    },
  };
  occurrences_section.body.push(Line {
    text: format!("- [{}]({})", title, &path),
  });

  let line = occurrences_section.line_number;
  let end = occurrences_section.title_line.text.len() as u32;
  doc.save(&base_dir);
  Fixed(AddedOccurrencesSection {
    location: Location {
      file: location.file,
      line,
      start: 0,
      end,
    },
    target: path,
  })
}

/// tries to extract a shortcut defined by the given regex from the given title
fn extract_shortcut<'a>(title: &'a str, regex: &Regex) -> ExtractShortcutResult<'a> {
  match regex.captures_len() {
    0 | 1 => ExtractShortcutResult::Failed(TitleRegexNoCaptures {
      regex: regex.to_string(),
    }),
    2 => match regex.captures(title) {
      None => ExtractShortcutResult::NoShortcutFound,
      Some(captures) => ExtractShortcutResult::ShortcutFound(captures.get(1).unwrap().as_str()),
    },
    other => ExtractShortcutResult::Failed(TitleRegexTooManyCaptures {
      regex: regex.to_string(),
      captures: other - 1,
    }),
  }
}

#[derive(Debug, PartialEq)]
enum ExtractShortcutResult<'a> {
  /// found a shortcut
  ShortcutFound(&'a str),
  /// the given title doesn't contain a shortcut
  NoShortcutFound,
  /// problem with the given Regex
  Failed(Issue),
}

/// removes all links from the given string
fn strip_links(text: &str) -> Cow<str> {
  let matches: Vec<Captures> = SOURCE_REGEX.captures_iter(text).collect();
  if matches.is_empty() {
    return Cow::Borrowed(text);
  }
  let mut result = text.to_owned();
  for m in matches {
    result = result.replace(m.get(0).unwrap().as_str(), m.get(1).unwrap().as_str());
  }
  Cow::Owned(result)
}
static SOURCE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[([^]]*)\]\([^)]*\)").unwrap());

#[cfg(test)]
mod tests {

  mod extract_shortcut {
    use crate::check::Issue;
    use crate::fix::missing_links::{extract_shortcut, ExtractShortcutResult};
    use big_s::S;
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
      let want = ExtractShortcutResult::Failed(Issue::TitleRegexNoCaptures { regex: S("123") });
      let have = extract_shortcut(give, &regex);
      assert_eq!(have, want);
    }

    #[test]
    fn regex_two_captures() {
      let regex = Regex::new("(\\w) (\\w)").unwrap();
      let give = "# Example Title";
      let want = ExtractShortcutResult::Failed(Issue::TitleRegexTooManyCaptures {
        regex: S("(\\w) (\\w)"),
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
