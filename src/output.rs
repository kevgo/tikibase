//! Tooling for outputting the results of lint operations in text/JSON format.

use crate::Fix;
use crate::check::Issue;
use crate::commands::Outcome;
use crate::search;
use big_s::S;
use serde::Serialize;

/// human-readable summary of running a single command
#[derive(Debug, Default, Eq, PartialEq, Serialize)]
pub struct Message {
  pub text: String,
  pub file: String,
  pub line: Option<u32>,
  pub start: Option<u32>,
  pub end: Option<u32>,
  pub fixable: bool,
}

impl Message {
  /// provides the CLI text format for this Message
  #[must_use]
  pub fn to_text(&self) -> String {
    if let Some(line) = self.line {
      format!("{}:{}  {}", self.file, line + 1, self.text)
    } else {
      format!("{}  {}", self.file, self.text)
    }
  }

  /// provides a Message instance summarizing the given Fix
  #[must_use]
  pub fn from_fix(fix: Fix) -> Self {
    match fix {
      Fix::RemovedEmptySection { title, location } => Self {
        text: format!("removed empty section \"{title}\""),
        file: location.file,
        line: Some(location.line),
        start: Some(location.start),
        end: Some(location.end),
        fixable: false,
      },
      Fix::AddedOccurrencesSection { location, target } => Self {
        text: format!("added {target} to occurrences section"),
        file: location.file,
        line: Some(location.line),
        start: Some(location.start),
        end: Some(location.end),
        fixable: false,
      },
      Fix::NormalizedSectionCapitalization {
        location,
        old_capitalization,
        new_capitalization,
      } => Self {
        text: format!(
          r#"normalized capitalization of section "{old_capitalization}" to "{new_capitalization}""#
        ),
        file: location.file,
        line: Some(location.line),
        start: Some(location.start),
        end: Some(location.end),
        fixable: false,
      },
      Fix::NormalizedSectionLevel {
        location,
        section_human_title,
        old_level,
        new_level,
      } => Self {
        text: format!(
          r#"normalized section "{section_human_title}" from <h{old_level}> to <h{new_level}>"#
        ),
        file: location.file,
        line: Some(location.line),
        start: Some(location.start),
        end: Some(location.end),
        fixable: false,
      },
      Fix::RemovedObsoleteOccurrencesSection { location } => Self {
        text: S("removed obsolete occurrences section"),
        file: location.file,
        line: Some(location.line),
        start: Some(location.start),
        end: Some(location.end),
        fixable: false,
      },
      Fix::SortedSections { location } => Self {
        text: S("fixed section order"),
        file: location.file,
        line: Some(location.line),
        start: Some(location.start),
        end: Some(location.end),
        fixable: false,
      },
    }
  }

  /// provides a Message instance summarizing the given Issue
  #[must_use]
  pub fn from_issue(issue: Issue) -> Self {
    match issue {
      Issue::BrokenImage { location, target } => Self {
        text: format!("image link to non-existing file \"{target}\""),
        file: location.file,
        line: Some(location.line),
        start: Some(location.start),
        end: Some(location.end),
        fixable: false,
      },
      Issue::CannotReadConfigurationFile { location, message } => Self {
        text: format!(
          "cannot read configuration file \"{}\": {}",
          location.file, message
        ),
        file: location.file,
        line: Some(location.line),
        start: Some(location.start),
        end: Some(location.end),
        fixable: false,
      },
      Issue::CannotReadDirectory { path, err } => Self {
        text: format!("cannot read directory: {err}"),
        file: path,
        line: None,
        start: None,
        end: None,
        fixable: false,
      },
      Issue::CannotWriteConfigFile { file, message } => Self {
        text: format!("cannot create configuration file: {message}"),
        file,
        line: None,
        start: None,
        end: None,
        fixable: false,
      },
      Issue::CannotWriteJsonSchemaFile { file, message } => Self {
        text: format!("cannot write JSON Schema file: {message}"),
        file,
        line: None,
        start: None,
        end: None,
        fixable: false,
      },
      Issue::DocumentWithoutLinks { location } => Self {
        text: S("document is not connected to any other documents"),
        file: location.file,
        line: Some(location.line),
        start: Some(location.start),
        end: Some(location.end),
        fixable: false,
      },
      Issue::DuplicateSection { location, title } => Self {
        text: format!("document contains multiple \"{title}\" sections"),
        file: location.file,
        line: Some(location.line),
        start: Some(location.start),
        end: Some(location.end),
        fixable: false,
      },
      Issue::EmptyDocument { path } => Self {
        text: S("no content"),
        file: path,
        line: None,
        start: None,
        end: None,
        fixable: false,
      },
      Issue::EmptySection { location, title } => Self {
        text: format!("section \"{title}\" has no content"),
        file: location.file,
        line: Some(location.line),
        start: Some(location.start),
        end: Some(location.end),
        fixable: true,
      },
      Issue::HeadingLevelDifferentThanConfigured {
        location,
        configured_level,
        configured_title: _,
        actual_level,
        actual_title,
      } => Self {
        text: format!(
          "heading level (<h{actual_level}>) of \"{actual_title}\" differs from configured level (<h{configured_level}>)"
        ),
        file: location.file,
        line: Some(location.line),
        start: Some(location.start),
        end: Some(location.end),
        fixable: true,
      },
      Issue::InconsistentHeadingLevel {
        location,
        section_title,
        common_level: common_variant,
        this_level: this_variant,
        all_levels: all_variants,
      } => {
        if let Some(common_variant) = common_variant {
          Self {
            text: format!(
              "heading level (<h{this_variant}>) is inconsistent with the usual level for \"{section_title}\" (<h{common_variant}>)"
            ),
            file: location.file,
            line: Some(location.line),
            start: Some(location.start),
            end: Some(location.end),
            fixable: true,
          }
        } else {
          let variants = all_variants
            .into_iter()
            .map(|e| format!("<h{e}>"))
            .collect::<Vec<String>>()
            .join(" and ");
          Self {
            text: format!(
              "inconsistent heading level - section \"{section_title}\" exists as {variants}"
            ),
            file: location.file,
            line: Some(location.line),
            start: Some(location.start),
            end: Some(location.end),
            fixable: true,
          }
        }
      }
      Issue::InvalidConfigurationFile { location, message } => Self {
        text: format!("invalid configuration file structure: {message}"),
        file: location.file,
        line: Some(location.line),
        start: Some(location.start),
        end: Some(location.end),
        fixable: false,
      },
      Issue::InvalidGlob {
        location,
        glob,
        message,
      } => Self {
        text: format!("invalid glob expression \"{glob}\": {message}"),
        file: location.file,
        line: Some(location.line),
        start: Some(location.start),
        end: Some(location.end),
        fixable: false,
      },
      Issue::InvalidTitleRegex {
        regex,
        problem,
        file,
      } => Self {
        text: format!(
          "Invalid regular expression in the \"titleRegEx\" entry ({regex}): {problem}"
        ),
        file,
        line: None,
        start: None,
        end: None,
        fixable: false,
      },
      Issue::LinkToNonExistingAnchorInCurrentDocument { location, anchor } => Self {
        text: format!("link to non-existing anchor \"{anchor}\" in current file"),
        file: location.file,
        line: Some(location.line),
        start: Some(location.start),
        end: Some(location.end),
        fixable: false,
      },
      Issue::LinkToNonExistingAnchorInExistingDocument {
        location,
        target_file,
        anchor,
      } => Self {
        text: format!("link to non-existing anchor \"{anchor}\" in \"{target_file}\""),
        file: location.file,
        line: Some(location.line),
        start: Some(location.start),
        end: Some(location.end),
        fixable: false,
      },
      Issue::LinkToNonExistingDir { location, target } => Self {
        text: format!("link to non-existing directory \"{target}\""),
        file: location.file,
        line: Some(location.line),
        start: Some(location.start),
        end: Some(location.end),
        fixable: false,
      },
      Issue::LinkToNonExistingFile { location, target } => Self {
        text: format!("link to non-existing file \"{target}\""),
        file: location.file,
        line: Some(location.line),
        start: Some(location.start),
        end: Some(location.end),
        fixable: false,
      },
      Issue::LinkToSameDocument { location } => Self {
        text: S("document contains link to itself"),
        file: location.file,
        line: Some(location.line),
        start: Some(location.start),
        end: Some(location.end),
        fixable: false,
      },
      Issue::LinkWithoutTarget { location } => Self {
        text: S("link without target"),
        file: location.file,
        line: Some(location.line),
        start: Some(location.start),
        end: Some(location.end),
        fixable: false,
      },
      Issue::MissingLink {
        location,
        path,
        title: _,
      } => Self {
        text: format!("missing link to {path}"),
        file: location.file,
        line: Some(location.line),
        start: Some(location.start),
        end: Some(location.end),
        fixable: true,
      },
      Issue::MissingFootnote {
        location,
        identifier: index,
      } => Self {
        text: format!("footnote [^{index}] doesn't exist"),
        file: location.file,
        line: Some(location.line),
        start: Some(location.start),
        end: Some(location.end),
        fixable: false,
      },
      Issue::MixCapSection {
        location,
        all_variants,
        this_variant,
        common_variant,
        section_level: _,
      } => {
        if let Some(common_variant) = common_variant {
          Self {
            text: format!(
              r#"section capitalization ("{this_variant}") is inconsistent with the usual form "{common_variant}""#
            ),
            file: location.file,
            line: Some(location.line),
            start: Some(location.start),
            end: Some(location.end),
            fixable: false,
          }
        } else {
          Self {
            text: format!(
              "section title occurs with inconsistent capitalization: {}",
              all_variants.join("|")
            ),
            file: location.file,
            line: Some(location.line),
            start: Some(location.start),
            end: Some(location.end),
            fixable: false,
          }
        }
      }
      Issue::NoTitleSection { location } => Self {
        text: S("no title section"),
        file: location.file,
        line: Some(location.line),
        start: Some(location.start),
        end: Some(location.end),
        fixable: false,
      },
      Issue::ObsoleteOccurrencesSection { location } => Self {
        text: S("obsolete \"occurrences\" section"),
        file: location.file,
        line: Some(location.line),
        start: Some(location.start),
        end: Some(location.end),
        fixable: true,
      },
      Issue::OrphanedResource { location } => Self {
        text: S("file isn't linked to"),
        file: location.file,
        line: Some(location.line),
        start: Some(location.start),
        end: Some(location.end),
        fixable: false,
      },
      Issue::SectionWithoutHeader { location } => Self {
        text: S("section with empty title"),
        file: location.file,
        line: Some(location.line),
        start: Some(location.start),
        end: Some(location.end),
        fixable: false,
      },
      Issue::TitleRegexNoCaptures { regex } => Self {
        text: format!(
          "The regular expression in the \"titleRegEx\" entry ({regex}) doesn't contain a capture group"
        ),
        file: S("tikibase.json"),
        line: None,
        start: None,
        end: None,
        fixable: false,
      },
      Issue::TitleRegexTooManyCaptures { regex, captures } => Self {
        text: format!(
          "The regular expression in the \"titleRegEx\" entry ({regex}) should have only one capture group but has {captures}"
        ),
        file: S("tikibase.json"),
        line: None,
        start: None,
        end: None,
        fixable: false,
      },
      Issue::UnclosedBacktick { location } => Self {
        text: S("unclosed backtick"),
        file: location.file,
        line: Some(location.line),
        start: Some(location.start),
        end: Some(location.end),
        fixable: false,
      },
      Issue::UnclosedFence { location } => Self {
        text: S("unclosed fence"),
        file: location.file,
        line: Some(location.line),
        start: Some(location.start),
        end: Some(location.end),
        fixable: false,
      },
      Issue::UnknownSection {
        location,
        title,
        allowed_titles: allowed_types,
      } => {
        let alloweds: Vec<String> = allowed_types
          .iter()
          .map(|allowed| format!("\n  - {allowed}"))
          .collect();
        Self {
          text: format!(
            "section \"{}\" isn't listed in tikibase.json, allowed sections:{}",
            title,
            alloweds.join("")
          ),
          file: location.file,
          line: Some(location.line),
          start: Some(location.start),
          end: Some(location.end),
          fixable: false,
        }
      }
      Issue::UnorderedSections { location } => Self {
        text: S("sections occur in different order than specified by tikibase.json"),
        file: location.file,
        line: Some(location.line),
        start: Some(location.start),
        end: Some(location.end),
        fixable: true,
      },
      Issue::UnusedFootnote {
        location,
        identifier,
      } => Self {
        text: format!("unused footnote [^{identifier}]"),
        file: location.file,
        line: Some(location.line),
        start: Some(location.start),
        end: Some(location.end),
        fixable: false,
      },
    }
  }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Messages {
  /// messages for identified issues
  pub issues: Vec<Message>,
  /// messages for fixed issues
  pub fixes: Vec<Message>,
  pub search_results: Vec<search::Result>,
  pub exit_code: u8,
}

impl Messages {
  /// provides the combined set of issues and fixes
  #[must_use]
  pub fn all(mut self) -> Vec<Message> {
    let mut result = vec![];
    result.append(&mut self.issues);
    result.append(&mut self.fixes);
    result
  }

  /// indicates whether there are any messages
  #[must_use]
  pub fn is_empty(&self) -> bool {
    self.issues.is_empty() && self.fixes.is_empty()
  }

  #[must_use]
  pub fn from_issue(issue: Issue) -> Self {
    Self {
      issues: vec![Message::from_issue(issue)],
      fixes: vec![],
      search_results: vec![],
      exit_code: 1,
    }
  }

  #[must_use]
  pub fn from_issues(issues: Vec<Issue>) -> Self {
    let exit_code = issues.len() as u8;
    Self {
      issues: issues.into_iter().map(Message::from_issue).collect(),
      fixes: vec![],
      search_results: vec![],
      exit_code,
    }
  }

  pub fn from_outcome(outcome: Outcome) -> Self {
    Self {
      fixes: outcome.fixes.into_iter().map(Message::from_fix).collect(),
      ..Self::from_issues(outcome.issues)
    }
  }

  /// indicates whether there are both issues and fixes
  #[must_use]
  pub fn has_issues_and_fixes(&self) -> bool {
    !self.issues.is_empty() && !self.fixes.is_empty()
  }
}

#[cfg(test)]
mod tests {

  mod all {
    use crate::Messages;
    use crate::output::Message;
    use big_s::S;

    #[test]
    fn empty() {
      let give = Messages::default();
      let want: Vec<Message> = vec![];
      let have = give.all();
      assert_eq!(have, want);
    }

    #[test]
    fn with_content() {
      let give = Messages {
        issues: vec![
          Message {
            text: S("issue 1"),
            ..Message::default()
          },
          Message {
            text: S("issue 2"),
            ..Message::default()
          },
        ],
        fixes: vec![
          Message {
            text: S("fix 1"),
            ..Message::default()
          },
          Message {
            text: S("fix 2"),
            ..Message::default()
          },
        ],
        ..Messages::default()
      };
      let result = give.all();
      let have: Vec<String> = result.into_iter().map(|message| message.text).collect();
      let want = vec!["issue 1", "issue 2", "fix 1", "fix 2"];
      assert_eq!(have, want);
    }
  }

  mod is_empty {
    use crate::Messages;
    use crate::output::Message;

    #[test]
    fn empty() {
      let give = Messages::default();
      assert!(give.is_empty());
    }

    #[test]
    fn with_issues() {
      let give = Messages {
        issues: vec![Message::default()],
        ..Messages::default()
      };
      assert!(!give.is_empty());
    }

    #[test]
    fn with_fixes() {
      let give = Messages {
        fixes: vec![Message::default()],
        ..Messages::default()
      };
      assert!(!give.is_empty());
    }

    #[test]
    fn with_issues_and_fixes() {
      let give = Messages {
        fixes: vec![Message::default()],
        issues: vec![Message::default()],
        ..Messages::default()
      };
      assert!(!give.is_empty());
    }
  }

  mod has_issues_and_fixes {
    use crate::Messages;
    use crate::output::Message;

    #[test]
    fn empty() {
      let give = Messages::default();
      assert!(!give.has_issues_and_fixes());
    }

    #[test]
    fn issues_only() {
      let give = Messages {
        issues: vec![Message::default()],
        ..Messages::default()
      };
      assert!(!give.has_issues_and_fixes());
    }

    #[test]
    fn fixes_only() {
      let give = Messages {
        fixes: vec![Message::default()],
        ..Messages::default()
      };
      assert!(!give.has_issues_and_fixes());
    }

    #[test]
    fn issues_and_fixes() {
      let give = Messages {
        fixes: vec![Message::default()],
        issues: vec![Message::default()],
        ..Messages::default()
      };
      assert!(give.has_issues_and_fixes());
    }
  }
}
