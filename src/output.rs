//! Tooling for outputting the results of lint operations in text/JSON format.

use crate::{Fix, Issue, Outcome, Position};
use serde::Serialize;
use std::borrow::Cow;

/// human-readable summary of running a single command
#[derive(Debug, PartialEq, Serialize)]
pub struct Message {
    pub pos: Position,
    pub text: String,
}

impl Message {
    /// provides the CLI text format for this Message
    pub fn to_text(&self) -> String {
        format!(
            "{}:{}  {}",
            self.pos.file.to_string_lossy(),
            self.pos.line + 1,
            self.text
        )
    }

    /// provides a Message instance summarizing the given Fix
    pub fn from_fix(fix: Fix) -> Message {
        match fix {
            Fix::RemovedEmptySection { section_type, pos } => Message {
                text: format!("removed empty section \"{}\"", section_type),
                pos,
            },
            Fix::AddedOccurrencesSection { pos } => Message {
                text: "added occurrences section".into(),
                pos,
            },
            Fix::RemovedObsoleteOccurrencesSection { pos } => Message {
                text: "removed obsolete occurrences section".into(),
                pos,
            },
            Fix::SortedSections { pos } => Message {
                text: "fixed section order".into(),
                pos,
            },
        }
    }

    /// provides a Message instance summarizing the given Issue
    pub fn from_issue(issue: Issue) -> Message {
        match issue {
            Issue::BrokenImage { pos, target } => Message {
                text: format!("image link to non-existing file \"{}\"", target),
                pos,
            },
            Issue::BrokenLink { pos, target } => Message {
                text: format!("link to non-existing file \"{}\"", target),
                pos,
            },
            Issue::CannotReadConfigurationFile { pos, message } => Message {
                text: format!(
                    "cannot read configuration file \"{}\": {}",
                    pos.file.to_string_lossy(),
                    message
                ),
                pos,
            },
            Issue::DuplicateSection { pos, section_type } => Message {
                text: format!("document contains multiple \"{}\" sections", section_type),
                pos,
            },
            Issue::EmptySection { pos, section_type } => Message {
                text: format!("section \"{}\" has no content", section_type),
                pos,
            },
            Issue::InvalidConfigurationFile { pos, message } => Message {
                text: format!(
                    "tikibase.json  invalid configuration file structure: {}",
                    message
                ),
                pos,
            },
            Issue::LinkToSameDocument { pos } => Message {
                text: "document contains link to itself".into(),
                pos,
            },
            Issue::LinkWithoutDestination { pos } => Message {
                text: "link without destination".into(),
                pos,
            },
            Issue::MissingLinks { pos, links } => {
                let links: Vec<Cow<str>> =
                    links.iter().map(|ml| ml.path.to_string_lossy()).collect();
                Message {
                    text: format!("missing link to {}", links.join(", ")),
                    pos,
                }
            }
            Issue::MissingSource { pos, index } => Message {
                text: format!("source [{}] doesn't exist", index),
                pos,
            },
            Issue::MixCapSection { pos, variants } => Message {
                text: format!(
                    "section title occurs with inconsistent capitalization: {}",
                    variants.join("|")
                ),
                pos,
            },
            Issue::NoTitleSection { pos } => Message {
                text: "no title section".into(),
                pos,
            },
            Issue::ObsoleteOccurrencesSection { pos } => Message {
                text: "obsolete \"occurrences\" section".into(),
                pos,
            },
            Issue::OrphanedResource { pos } => Message {
                text: "file isn't linked to".into(),
                pos,
            },
            Issue::SectionWithoutHeader { pos } => Message {
                text: "section with empty title".into(),
                pos,
            },
            Issue::UnclosedFence { pos } => Message {
                text: "unclosed fence".into(),
                pos,
            },
            Issue::UnknownSection {
                pos,
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
                    pos,
                }
            }
            Issue::UnorderedSections { pos } => Message {
                text: "sections occur in different order than specified by tikibase.json".into(),
                pos,
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
