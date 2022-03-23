//! Tooling for outputting the results of lint operations in text/JSON format.

use crate::{Fix, Issue, Outcome};
use serde::Serialize;
use std::borrow::Cow;
use std::path::PathBuf;

/// human-readable summary of running a single command
#[derive(Debug, PartialEq, Serialize)]
pub struct Message {
    pub text: String,
    pub file: PathBuf,
    pub line: u32,
    pub start: u32,
    pub end: u32,
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
            },
            Fix::AddedOccurrencesSection { location } => Message {
                text: "added occurrences section".into(),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
            },
            Fix::RemovedObsoleteOccurrencesSection { location } => Message {
                text: "removed obsolete occurrences section".into(),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
            },
            Fix::SortedSections { location } => Message {
                text: "fixed section order".into(),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
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
            },
            Issue::BrokenLink { location, target } => Message {
                text: format!("link to non-existing file \"{}\"", target),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
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
            },
            Issue::DuplicateSection { location, title } => Message {
                text: format!("document contains multiple \"{}\" sections", title),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
            },
            Issue::EmptySection { location, title } => Message {
                text: format!("section \"{}\" has no content", title),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
            },
            Issue::InvalidConfigurationFile { location, message } => Message {
                text: format!(
                    "tikibase.json  invalid configuration file structure: {}",
                    message
                ),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
            },
            Issue::LinkToSameDocument { location } => Message {
                text: "document contains link to itself".into(),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
            },
            Issue::LinkWithoutDestination { location } => Message {
                text: "link without destination".into(),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
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
                }
            }
            Issue::MissingSource {
                location,
                identifier: index,
            } => Message {
                text: format!("source [{}] doesn't exist", index),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
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
            },
            Issue::NoTitleSection { location } => Message {
                text: "no title section".into(),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
            },
            Issue::ObsoleteOccurrencesSection { location } => Message {
                text: "obsolete \"occurrences\" section".into(),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
            },
            Issue::OrphanedResource { location } => Message {
                text: "file isn't linked to".into(),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
            },
            Issue::SectionWithoutHeader { location } => Message {
                text: "section with empty title".into(),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
            },
            Issue::UnclosedFence { location } => Message {
                text: "unclosed fence".into(),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
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
                }
            }
            Issue::UnorderedSections { location } => Message {
                text: "sections occur in different order than specified by tikibase.json".into(),
                file: location.file,
                line: location.line,
                start: location.start,
                end: location.end,
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
