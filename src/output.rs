//! The outer API of Tikibase. It provides the results of a full Tikibase run
//! including human-readable summaries of what Tikibase has done.

use crate::{Fix, Issue, Outcome};
use serde::Serialize;
use std::borrow::Cow;

/// human-readable summary of running a single command
#[derive(Debug, PartialEq, Serialize)]
pub struct Message {
    pub file: Option<String>,
    pub line: Option<u32>,
    pub text: String,
}

impl Message {
    /// provides the CLI text format for this Message
    pub fn to_text(&self) -> String {
        match (&self.file, self.line) {
            (Some(file), Some(line)) => {
                format!("{}:{}  {}", file, line + 1, self.text)
            }
            (Some(file), None) => format!("{}  {}", file, self.text),
            (None, None) => self.text.clone(),
            (None, Some(_line)) => panic!("should never get just a line without a file"),
        }
    }

    /// provides a Message instance summarizing the given Fix
    fn from_fix(fix: Fix) -> Message {
        match fix {
            Fix::RemovedEmptySection {
                section_type,
                file,
                line,
            } => Message {
                text: format!("removed empty section \"{}\"", section_type),
                file: Some(file.to_string_lossy().to_string()),
                line: Some(line),
            },
            Fix::AddedOccurrencesSection { file, line } => Message {
                text: "added occurrences section".into(),
                file: Some(file.to_string_lossy().to_string()),
                line: Some(line),
            },
            Fix::RemovedObsoleteOccurrencesSection { file, line } => Message {
                text: "removed obsolete occurrences section".into(),
                file: Some(file.to_string_lossy().to_string()),
                line: Some(line),
            },
            Fix::SortedSections { file } => Message {
                text: "fixed section order".into(),
                file: Some(file.to_string_lossy().to_string()),
                line: None,
            },
        }
    }

    /// provides a Message instance summarizing the given Issue
    pub fn from_issue(issue: Issue) -> Message {
        match issue {
            Issue::BrokenImage { file, line, target } => Message {
                text: format!("image link to non-existing file \"{}\"", target),
                file: Some(file.to_string_lossy().to_string()),
                line: Some(line),
            },
            Issue::BrokenLink { file, line, target } => Message {
                text: format!("link to non-existing file \"{}\"", target),
                file: Some(file.to_string_lossy().to_string()),
                line: Some(line),
            },
            Issue::CannotReadConfigurationFile { message } => Message {
                text: format!(
                    "cannot read configuration file \"tikibase.json\": {}",
                    message
                ),
                file: None,
                line: None,
            },
            Issue::DuplicateSection { file, section_type } => Message {
                text: format!("document contains multiple \"{}\" sections", section_type),
                file: Some(file.to_string_lossy().to_string()),
                line: None,
            },
            Issue::EmptySection {
                file,
                line,
                section_type,
            } => Message {
                text: format!("section \"{}\" has no content", section_type),
                file: Some(file.to_string_lossy().to_string()),
                line: Some(line),
            },
            Issue::InvalidConfigurationFile { message } => Message {
                text: format!(
                    "tikibase.json  invalid configuration file structure: {}",
                    message
                ),
                file: None,
                line: None,
            },
            Issue::LinkToSameDocument { file, line } => Message {
                text: "document contains link to itself".into(),
                file: Some(file.to_string_lossy().to_string()),
                line: Some(line),
            },
            Issue::LinkWithoutDestination { file, line } => Message {
                text: "link without destination".into(),
                file: Some(file.to_string_lossy().to_string()),
                line: Some(line),
            },
            Issue::MissingLinks { file, links } => {
                let links: Vec<Cow<str>> =
                    links.iter().map(|ml| ml.path.to_string_lossy()).collect();
                Message {
                    text: format!("missing link to {}", links.join(", ")),
                    file: Some(file.to_string_lossy().to_string()),
                    line: None,
                }
            }
            Issue::MissingSource { file, line, index } => Message {
                text: format!("source [{}] doesn't exist", index),
                file: Some(file.to_string_lossy().to_string()),
                line: Some(line),
            },
            Issue::MixCapSection { variants } => Message {
                text: format!(
                    "section title occurs with inconsistent capitalization: {}",
                    variants.join("|")
                ),
                file: None,
                line: None,
            },
            Issue::NoTitleSection { file } => Message {
                text: "no title section".into(),
                file: Some(file.to_string_lossy().to_string()),
                line: None,
            },
            Issue::ObsoleteOccurrencesSection { file, line } => Message {
                text: "obsolete \"occurrences\" section".into(),
                file: Some(file.to_string_lossy().to_string()),
                line: Some(line),
            },
            Issue::OrphanedResource { path } => Message {
                text: "file isn't linked to".into(),
                file: Some(path),
                line: None,
            },
            Issue::SectionWithoutHeader { file, line } => Message {
                text: "section with empty title".into(),
                file: Some(file.to_string_lossy().to_string()),
                line: Some(line),
            },
            Issue::UnclosedFence { file, line } => Message {
                text: "unclosed fence".into(),
                file: Some(file.to_string_lossy().to_string()),
                line: Some(line),
            },
            Issue::UnknownSection {
                file,
                line,
                section_type,
                allowed_types,
            } => {
                let alloweds: Vec<String> = allowed_types
                    .iter()
                    .map(|allowed| format!("\n  - {}", allowed))
                    .collect();
                Message {
                    text: format!(
                        "section \"{}\" isn't listed in tikibase.json, allowed sections:{}",
                        section_type,
                        alloweds.join("")
                    ),
                    file: Some(file.to_string_lossy().to_string()),
                    line: Some(line),
                }
            }
            Issue::UnorderedSections { file } => Message {
                text: "sections occur in different order than specified by tikibase.json".into(),
                file: Some(file.to_string_lossy().to_string()),
                line: None,
            },
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Messages {
    pub messages: Vec<Message>,
    pub exit_code: i32,
}

impl Messages {
    pub fn from_issue(issue: Issue) -> Messages {
        Messages {
            messages: vec![Message::from_issue(issue)],
            exit_code: 1,
        }
    }
    pub fn from_issues(issues: Vec<Issue>) -> Messages {
        Messages {
            exit_code: issues.len() as i32,
            messages: issues.into_iter().map(Message::from_issue).collect(),
        }
    }

    pub fn from_outcome(outcome: Outcome) -> Messages {
        let exit_code = outcome.issues.len() as i32;
        let mut messages = vec![];
        messages.extend(outcome.fixes.into_iter().map(Message::from_fix));
        messages.extend(outcome.issues.into_iter().map(Message::from_issue));
        Messages {
            messages,
            exit_code,
        }
    }
}
