//! Tooling for outputting the results of lint operations in text/JSON format.

use crate::{Fix, Issue, Outcome};
use serde::Serialize;
use std::borrow::Cow;
use std::path::PathBuf;

/// human-readable summary of running a single command
#[derive(Debug, Default, PartialEq, Serialize)]
pub struct Message {
    pub text: String,
    pub file: PathBuf,
    pub line: u32,
    pub start: u32,
    pub end: u32,
    pub fixable: bool,
}

impl Message {
    /// provides the CLI text format for this Message
    pub fn to_text(&self) -> String {
        format!(
            "{}:{}  {}",
            self.file.to_string_lossy(),
            self.line + 1,
            self.text
        )
    }

    /// provides a Message instance summarizing the given Fix
    pub fn from_fix(fix: Fix) -> Message {
        match fix {
            Fix::RemovedEmptySection { title, location } => Message {
                text: format!("removed empty section \"{}\"", title),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
                fixable: false,
            },
            Fix::AddedOccurrencesSection { location } => Message {
                text: "added occurrences section".into(),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
                fixable: false,
            },
            Fix::RemovedObsoleteOccurrencesSection { location } => Message {
                text: "removed obsolete occurrences section".into(),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
                fixable: false,
            },
            Fix::SortedSections { location } => Message {
                text: "fixed section order".into(),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
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
                line: location.line,
                start: location.start,
                end: location.end,
                fixable: false,
            },
            Issue::CannotReadConfigurationFile { location, message } => Message {
                text: format!(
                    "cannot read configuration file \"{}\": {}",
                    location.file.to_string_lossy(),
                    message
                ),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
                fixable: false,
            },
            Issue::CannotWriteConfigFile { message, location } => Message {
                text: format!("cannot write the example configuration file: {}", message),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
                fixable: false,
            },
            Issue::CannotWriteJsonSchemaFile { location, message } => Message {
                text: format!("cannot write JSON Schema file: {}", message),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
                fixable: false,
            },
            Issue::DocumentWithoutLinks { location } => Message {
                text: "document is not connected to any other documents".into(),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
                fixable: false,
            },
            Issue::DuplicateSection { location, title } => Message {
                text: format!("document contains multiple \"{}\" sections", title),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
                fixable: false,
            },
            Issue::EmptySection { location, title } => Message {
                text: format!("section \"{}\" has no content", title),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
                fixable: true,
            },
            Issue::InvalidConfigurationFile { location, message } => Message {
                text: format!("invalid configuration file structure: {}", message),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
                fixable: false,
            },
            Issue::InvalidGlob {
                location,
                glob,
                message,
            } => Message {
                text: format!("invalid glob expression \"{}\": {}", glob, message),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
                fixable: false,
            },
            Issue::LinkToNonExistingAnchorInCurrentDocument { location, anchor } => Message {
                text: format!(
                    "link to non-existing anchor \"#{}\" in current file",
                    anchor
                ),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
                fixable: false,
            },
            Issue::LinkToNonExistingAnchorInExistingDocument {
                location,
                target_file,
                anchor,
            } => Message {
                text: format!(
                    "link to non-existing anchor \"#{}\" in \"{}\"",
                    anchor, target_file
                ),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
                fixable: false,
            },
            Issue::LinkToNonExistingFile { location, target } => Message {
                text: format!("link to non-existing file \"{}\"", target),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
                fixable: false,
            },
            Issue::LinkToSameDocument { location } => Message {
                text: "document contains link to itself".into(),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
                fixable: false,
            },
            Issue::LinkWithoutTarget { location } => Message {
                text: "link without target".into(),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
                fixable: false,
            },
            Issue::MissingLinks { location, links } => {
                let links: Vec<Cow<str>> =
                    links.iter().map(|ml| ml.path.to_string_lossy()).collect();
                Message {
                    text: format!("missing link to {}", links.join(", ")),
                    file: location.file,
                    line: location.line,
                    start: location.start,
                    end: location.end,
                    fixable: true,
                }
            }
            Issue::MissingFootnote {
                location,
                identifier: index,
            } => Message {
                text: format!("footnote [^{}] doesn't exist", index),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
                fixable: false,
            },
            Issue::MixCapSection { location, variants } => Message {
                text: format!(
                    "section title occurs with inconsistent capitalization: {}",
                    variants.join("|")
                ),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
                fixable: false,
            },
            Issue::NoTitleSection { location } => Message {
                text: "no title section".into(),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
                fixable: false,
            },
            Issue::ObsoleteOccurrencesSection { location } => Message {
                text: "obsolete \"occurrences\" section".into(),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
                fixable: true,
            },
            Issue::OrphanedResource { location } => Message {
                text: "file isn't linked to".into(),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
                fixable: false,
            },
            Issue::SectionWithoutHeader { location } => Message {
                text: "section with empty title".into(),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
                fixable: false,
            },
            Issue::UnclosedBacktick { location } => Message {
                text: "unclosed backtick".into(),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
                fixable: false,
            },
            Issue::UnclosedFence { location } => Message {
                text: "unclosed fence".into(),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
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
                    line: location.line,
                    start: location.start,
                    end: location.end,
                    fixable: false,
                }
            }
            Issue::UnorderedSections { location } => Message {
                text: "sections occur in different order than specified by tikibase.json".into(),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
                fixable: true,
            },
            Issue::UnusedFootnote {
                location,
                identifier,
            } => Message {
                text: format!("unused footnote [^{}]", identifier),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
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
    pub exit_code: i32,
}

impl Messages {
    /// provides the combined set of issues and fixes
    pub fn all(mut self) -> Vec<Message> {
        let mut result = vec![];
        result.append(&mut self.issues);
        result.append(&mut self.fixes);
        result
    }
    pub fn from_issue(issue: Issue) -> Messages {
        Messages {
            issues: vec![Message::from_issue(issue)],
            fixes: vec![],
            exit_code: 1,
        }
    }
    pub fn from_issues(issues: Vec<Issue>) -> Messages {
        let exit_code = issues.len() as i32;
        Messages {
            issues: issues.into_iter().map(Message::from_issue).collect(),
            fixes: vec![],
            exit_code,
        }
    }

    pub fn from_outcome(outcome: Outcome) -> Messages {
        let exit_code = outcome.issues.len() as i32;
        Messages {
            issues: outcome
                .issues
                .into_iter()
                .map(Message::from_issue)
                .collect(),
            fixes: outcome.fixes.into_iter().map(Message::from_fix).collect(),
            exit_code,
        }
    }

    /// indicates whether there are any messages
    pub fn is_empty(&self) -> bool {
        self.issues.is_empty() && self.fixes.is_empty()
    }
}

#[cfg(test)]
mod tests {

    mod all {
        use crate::{Message, Messages};

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
        use crate::{Message, Messages};

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
    }
}
