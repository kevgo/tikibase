//! Tooling for outputting the results of lint operations in text/JSON format.

use crate::check::Issue;
use crate::commands::Outcome;
use crate::Fix;
use serde::Serialize;

/// human-readable summary of running a single command
#[derive(Debug, Default, PartialEq, Serialize)]
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
    pub fn to_text(&self) -> String {
        if let Some(line) = self.line {
            format!("{}:{}  {}", self.file, line + 1, self.text)
        } else {
            format!("{}  {}", self.file, self.text)
        }
    }

    /// provides a Message instance summarizing the given Fix
    pub fn from_fix(fix: Fix) -> Message {
        match fix {
            Fix::RemovedEmptySection { title, location } => Message {
                text: format!("removed empty section \"{}\"", title),
                file: location.file,
                line: Some(location.line),
                start: Some(location.start),
                end: Some(location.end),
                fixable: false,
            },
            Fix::AddedOccurrencesSection { location, target } => Message {
                text: format!("added {} to occurrences section", target),
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
            } => Message {
                text: format!(
                    r#"normalized capitalization of section "{}" to "{}""#,
                    old_capitalization, new_capitalization
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
            } => Message {
                text: format!(
                    r#"normalized section "{}" from <h{}> to <h{}>"#,
                    section_human_title, old_level, new_level
                ),
                file: location.file,
                line: Some(location.line),
                start: Some(location.start),
                end: Some(location.end),
                fixable: false,
            },
            Fix::RemovedObsoleteOccurrencesSection { location } => Message {
                text: "removed obsolete occurrences section".into(),
                file: location.file,
                line: Some(location.line),
                start: Some(location.start),
                end: Some(location.end),
                fixable: false,
            },
            Fix::SortedSections { location } => Message {
                text: "fixed section order".into(),
                file: location.file,
                line: Some(location.line),
                start: Some(location.start),
                end: Some(location.end),
                fixable: false,
            },
        }
    }

    /// provides a Message instance summarizing the given Issue
    pub fn from_issue(issue: Issue) -> Message {
        match issue {
            Issue::BrokenImage { location, target } => Message {
                text: format!("image link to non-existing file \"{}\"", target),
                file: location.file,
                line: Some(location.line),
                start: Some(location.start),
                end: Some(location.end),
                fixable: false,
            },
            Issue::CannotReadConfigurationFile { location, message } => Message {
                text: format!(
                    "cannot read configuration file \"{}\": {}",
                    location.file,
                    message
                ),
                file: location.file,
                line: Some(location.line),
                start: Some(location.start),
                end: Some(location.end),
                fixable: false,
            },
            Issue::CannotReadDirectory { path, err } => Message{
                text: format!("cannot read directory: {}", err),
                file: path,
                line: None,
                start: None,
                end: None,
                fixable: false
            },
            Issue::CannotWriteConfigFile { file, message } => Message {
                text: format!("cannot create configuration file: {}", message),
                file,
                line: None,
                start: None,
                end: None,
                fixable: false,
            },
            Issue::CannotWriteJsonSchemaFile { file, message } => Message {
                text: format!("cannot write JSON Schema file: {}", message),
                file,
                line: None,
                start: None,
                end: None,
                fixable: false,
            },
            Issue::DocumentWithoutLinks { location } => Message {
                text: "document is not connected to any other documents".into(),
                file: location.file,
                line: Some(location.line),
                start: Some(location.start),
                end: Some(location.end),
                fixable: false,
            },
            Issue::DuplicateSection { location, title } => Message {
                text: format!("document contains multiple \"{}\" sections", title),
                file: location.file,
                line: Some(location.line),
                start: Some(location.start),
                end: Some(location.end),
                fixable: false,
            },
            Issue::EmptyDocument { path} => Message{
                text: "no content".into(),
                file: path,
                line: None,
                start: None,
                end: None,
                fixable: false
            },
            Issue::EmptySection { location, title } => Message {
                text: format!("section \"{}\" has no content", title),
                file: location.file,
                line: Some(location.line),
                start: Some(location.start),
                end: Some(location.end),
                fixable: true,
            },
            Issue::HeadingLevelDifferentThanConfigured { location, configured_level, configured_title: _, actual_level, actual_title} => Message{
                text: format!("heading level (<h{}>) of \"{}\" differs from configured level (<h{}>)", actual_level, actual_title, configured_level),
                file: location.file,
                line: Some(location.line),
                start: Some(location.start),
                end: Some(location.end),
                fixable: true,
            },
            Issue::InconsistentHeadingLevel { location, section_title, common_level: common_variant, this_level: this_variant, all_levels: all_variants } => {
                if let Some(common_variant) = common_variant {
                    Message {
                        text: format!("heading level (<h{}>) is inconsistent with the usual level for \"{}\" (<h{}>)", this_variant, section_title, common_variant),
                        file: location.file,
                        line: Some(location.line),
                        start: Some(location.start),
                        end: Some(location.end),
                        fixable: true,
                    }
                } else {
                    let variants = all_variants.into_iter().map(|e| format!("<h{}>", e)).collect::<Vec<String>>().join(" and ");
                    Message {
                        text: format!("inconsistent heading level - section \"{}\" exists as {}", section_title, variants),
                        file: location.file,
                        line: Some(location.line),
                        start: Some(location.start),
                        end: Some(location.end),
                        fixable: true,
                    }
                }
            },
            Issue::InvalidConfigurationFile { location, message } => Message {
                text: format!("invalid configuration file structure: {}", message),
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
            } => Message {
                text: format!("invalid glob expression \"{}\": {}", glob, message),
                file: location.file,
                line: Some(location.line),
                start: Some(location.start),
                end: Some(location.end),
                fixable: false,
            },
            Issue::InvalidTitleRegex { regex, problem, file } => Message{
                text: format!("Invalid regular expression in the \"titleRegEx\" entry ({}): {}", regex, problem),
                file,
                line: None,
                start: None,
                end: None,
                fixable: false
            },
            Issue::LinkToNonExistingAnchorInCurrentDocument { location, anchor } => Message {
                text: format!(
                    "link to non-existing anchor \"{}\" in current file",
                    anchor
                ),
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
            } => Message {
                text: format!(
                    "link to non-existing anchor \"{}\" in \"{}\"",
                    anchor, target_file
                ),
                file: location.file,
                line: Some(location.line),
                start: Some(location.start),
                end: Some(location.end),
                fixable: false,
            },
            Issue::LinkToNonExistingDir { location, target} => Message {
                text: format!("link to non-existing directory \"{}\"", target),
                file: location.file,
                line: Some(location.line),
                start: Some(location.start),
                end: Some(location.end),
                fixable: false,
            },
            Issue::LinkToNonExistingFile { location, target } => Message {
                text: format!("link to non-existing file \"{}\"", target),
                file: location.file,
                line: Some(location.line),
                start: Some(location.start),
                end: Some(location.end),
                fixable: false,
            },
            Issue::LinkToSameDocument { location } => Message {
                text: "document contains link to itself".into(),
                file: location.file,
                line: Some(location.line),
                start: Some(location.start),
                end: Some(location.end),
                fixable: false,
            },
            Issue::LinkWithoutTarget { location } => Message {
                text: "link without target".into(),
                file: location.file,
                line: Some(location.line),
                start: Some(location.start),
                end: Some(location.end),
                fixable: false,
            },
            Issue::MissingLink { location, path, title: _ } => {
                Message {
                    text: format!("missing link to {}", path),
                    file: location.file,
                    line: Some(location.line),
                    start: Some(location.start),
                    end: Some(location.end),
                    fixable: true,
                }
            }
            Issue::MissingFootnote {
                location,
                identifier: index,
            } => Message {
                text: format!("footnote [^{}] doesn't exist", index),
                file: location.file,
                line: Some(location.line),
                start: Some(location.start),
                end: Some(location.end),
                fixable: false,
            },
            Issue::MixCapSection { location, all_variants, this_variant, common_variant, section_level: _ } => {
                if let Some(common_variant) = common_variant {
                    Message {
                        text: format!(
                            r#"section capitalization ("{}") is inconsistent with the usual form "{}""#,
                            this_variant, common_variant
                        ),
                        file: location.file,
                        line: Some(location.line),
                        start: Some(location.start),
                        end: Some(location.end),
                        fixable: false,
                    }
                } else {
                    Message {
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
            },
            Issue::NoTitleSection { location } => Message {
                text: "no title section".into(),
                file: location.file,
                line: Some(location.line),
                start: Some(location.start),
                end: Some(location.end),
                fixable: false,
            },
            Issue::ObsoleteOccurrencesSection { location } => Message {
                text: "obsolete \"occurrences\" section".into(),
                file: location.file,
                line: Some(location.line),
                start: Some(location.start),
                end: Some(location.end),
                fixable: true,
            },
            Issue::OrphanedResource { location } => Message {
                text: "file isn't linked to".into(),
                file: location.file,
                line: Some(location.line),
                start: Some(location.start),
                end: Some(location.end),
                fixable: false,
            },
            Issue::PathEscapesRoot { path, location } => Message {
                text: format!("The path \"{}\" goes above the root directory", path),
                file: location.file,
                line: Some(location.line),
                start: Some(location.start),
                end: Some(location.end),
                fixable: false,
            },
            Issue::SectionWithoutHeader { location } => Message {
                text: "section with empty title".into(),
                file: location.file,
                line: Some(location.line),
                start: Some(location.start),
                end: Some(location.end),
                fixable: false,
            },
            Issue::TitleRegexNoCaptures { regex } => Message {
                text: format!("The regular expression in the \"titleRegEx\" entry ({}) doesn't contain a capture group", regex),
                file: "tikibase.json".into(),
                line: None,
                start: None,
                end: None,
                fixable: false,
            },
            Issue::TitleRegexTooManyCaptures { regex, captures } => Message{
                text: format!("The regular expression in the \"titleRegEx\" entry ({}) should have only one capture group but has {}", regex, captures),
                file: "tikibase.json".into(),
                line: None,
                start: None,
                end: None,
                fixable: false,
            },
            Issue::UnclosedBacktick { location } => Message {
                text: "unclosed backtick".into(),
                file: location.file,
                line: Some(location.line),
                start: Some(location.start),
                end: Some(location.end),
                fixable: false,
            },
            Issue::UnclosedFence { location } => Message {
                text: "unclosed fence".into(),
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
                    .map(|allowed| format!("\n  - {}", allowed))
                    .collect();
                Message {
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
            Issue::UnorderedSections { location } => Message {
                text: "sections occur in different order than specified by tikibase.json".into(),
                file: location.file,
                line: Some(location.line),
                start: Some(location.start),
                end: Some(location.end),
                fixable: true,
            },
            Issue::UnusedFootnote {
                location,
                identifier,
            } => Message {
                text: format!("unused footnote [^{}]", identifier),
                file: location.file,
                line: Some(location.line),
                start: Some(location.start),
                end: Some(location.end),
                fixable: false,
            },
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Messages {
    /// messages for identified issues
    pub issues: Vec<Message>,
    /// messages for fixed issues
    pub fixes: Vec<Message>,
    pub exit_code: u8,
}

impl Messages {
    /// provides the combined set of issues and fixes
    pub fn all(mut self) -> Vec<Message> {
        let mut result = vec![];
        result.append(&mut self.issues);
        result.append(&mut self.fixes);
        result
    }

    /// indicates whether there are any messages
    pub fn is_empty(&self) -> bool {
        self.issues.is_empty() && self.fixes.is_empty()
    }

    pub fn from_issue(issue: Issue) -> Messages {
        Messages {
            issues: vec![Message::from_issue(issue)],
            fixes: vec![],
            exit_code: 1,
        }
    }
    pub fn from_issues(issues: Vec<Issue>) -> Messages {
        let exit_code = issues.len() as u8;
        Messages {
            issues: issues.into_iter().map(Message::from_issue).collect(),
            fixes: vec![],
            exit_code,
        }
    }

    pub fn from_outcome(outcome: Outcome) -> Messages {
        Messages {
            fixes: outcome.fixes.into_iter().map(Message::from_fix).collect(),
            ..Messages::from_issues(outcome.issues)
        }
    }

    /// indicates whether there are both issues and fixes
    pub fn has_issues_and_fixes(&self) -> bool {
        !self.issues.is_empty() && !self.fixes.is_empty()
    }
}

#[cfg(test)]
mod tests {

    mod all {
        use crate::output::Message;
        use crate::Messages;

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
                        text: "issue 1".into(),
                        ..Message::default()
                    },
                    Message {
                        text: "issue 2".into(),
                        ..Message::default()
                    },
                ],
                fixes: vec![
                    Message {
                        text: "fix 1".into(),
                        ..Message::default()
                    },
                    Message {
                        text: "fix 2".into(),
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
        use crate::output::Message;
        use crate::Messages;

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
        use crate::output::Message;
        use crate::Messages;

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
