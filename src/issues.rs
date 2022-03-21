use std::borrow::Cow;
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;

/// the issues that this linter can find
#[derive(Clone, Debug, PartialEq)]
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
        file: String,
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

#[derive(Clone, Debug, PartialEq)]
pub struct MissingLink {
    pub path: PathBuf,
    pub title: String,
}

impl Display for Issue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Issue::BrokenImage { file, line, target } => write!(
                f,
                "{}:{}  image link to non-existing file \"{}\"",
                file.to_string_lossy(),
                line,
                target
            ),
            Issue::BrokenLink { file, line, target } => write!(
                f,
                "{}:{}  link to non-existing file \"{}\"",
                file.to_string_lossy(),
                line,
                target
            ),
            Issue::CannotReadConfigurationFile { message: _ } => {
                write!(f, "cannot read configuration file \"tikibase.json\"")
            }
            Issue::DuplicateSection { file, section_type } => write!(
                f,
                "{}  document contains multiple \"{}\" sections",
                file.to_string_lossy(),
                section_type
            ),
            Issue::EmptySection {
                file,
                line,
                section_type,
            } => write!(
                f,
                "{}:{}  section \"{}\" has no content",
                file.to_string_lossy(),
                line + 1,
                section_type
            ),
            Issue::InvalidConfigurationFile { message } => {
                write!(
                    f,
                    "tikibase.json  invalid configuration file structure: {}",
                    message
                )
            }
            Issue::LinkToSameDocument { file, line } => write!(
                f,
                "{}:{}  document contains link to itself",
                file.to_string_lossy(),
                line
            ),
            Issue::LinkWithoutDestination { file, line } => write!(
                f,
                "{}:{}  link without destination",
                file.to_string_lossy(),
                line
            ),
            Issue::MissingLinks { file, links } => {
                let links: Vec<Cow<str>> =
                    links.iter().map(|ml| ml.path.to_string_lossy()).collect();
                write!(
                    f,
                    "{}  missing link to {}",
                    file.to_string_lossy(),
                    links.join(", "),
                )
            }
            Issue::MissingSource { file, line, index } => {
                write!(f, "{}:{}  source [{}] doesn't exist", file, line + 1, index)
            }
            Issue::MixCapSection { variants } => write!(
                f,
                "section title occurs with inconsistent capitalization: {}",
                variants.join("|")
            ),
            Issue::NoTitleSection { file } => {
                write!(f, "{}  no title section", file.to_string_lossy())
            }
            Issue::ObsoleteOccurrencesSection { file, line } => write!(
                f,
                "{}:{}  obsolete \"occurrences\" section",
                file.to_string_lossy(),
                line + 1,
            ),
            Issue::OrphanedResource { path } => write!(f, "file \"{}\" isn't linked to", path),
            Issue::SectionWithoutHeader { file, line } => write!(
                f,
                "{}:{}  section with empty title",
                file.to_string_lossy(),
                line + 1
            ),
            Issue::UnclosedFence { file, line } => {
                write!(f, "{}:{}  unclosed fence", file.to_string_lossy(), line + 1,)
            }
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
                write!(
                    f,
                    "{}:{}  section \"{}\" isn't listed in tikibase.json, allowed sections:{}",
                    file.to_string_lossy(),
                    line + 1,
                    section_type,
                    alloweds.join("")
                )
            }
            Issue::UnorderedSections { file } => {
                write!(
                    f,
                    "{}  sections occur in different order than specified by tikibase.json",
                    file.to_string_lossy()
                )
            }
        }
    }
}
