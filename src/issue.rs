use crate::Message;
use serde::Serialize;
use std::borrow::Cow;
use std::path::PathBuf;

/// the issues that this linter can find
/// NOTE: Since we are targeting human-readable knowledge bases here, all file paths are required to be valid unicode.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum Issue {
    BrokenImage {
        file: PathBuf,
        line: u32,
        target: String,
    },
    BrokenLink {
        file: PathBuf,
        line: u32,
        target: String,
    },
    CannotReadConfigurationFile {
        message: String,
    },
    DuplicateSection {
        file: PathBuf,
        section_type: String,
    },
    EmptySection {
        file: PathBuf,
        line: u32,
        section_type: String,
    },
    InvalidConfigurationFile {
        message: String,
    },
    LinkToSameDocument {
        file: PathBuf,
        line: u32,
    },
    LinkWithoutDestination {
        file: PathBuf,
        line: u32,
    },
    MissingLinks {
        file: PathBuf,
        links: Vec<MissingLink>,
    },
    MissingSource {
        file: PathBuf,
        line: u32,
        index: String,
    },
    MixCapSection {
        variants: Vec<String>,
    },
    NoTitleSection {
        file: PathBuf,
    },
    ObsoleteOccurrencesSection {
        file: PathBuf,
        line: u32,
    },
    OrphanedResource {
        // This is a String and not a Path because we need a String (to print it),
        // and we already converted the Path of this orphaned resource into a String
        // during processing it.
        path: String,
    },
    SectionWithoutHeader {
        file: PathBuf,
        line: u32,
    },
    UnclosedFence {
        file: PathBuf,
        line: u32,
    },
    UnknownSection {
        file: PathBuf,
        line: u32,
        section_type: String,
        allowed_types: Vec<String>,
    },
    UnorderedSections {
        file: PathBuf,
    },
}

/// a missing link to a document
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MissingLink {
    pub path: PathBuf,
    pub title: String,
}

impl Issue {
    pub fn to_message(self) -> Message {
        match self {
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
                text: format!("file \"{}\" isn't linked to", path),
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
