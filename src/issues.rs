use std::borrow::Cow;
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;

/// possible issues that this linter can find
pub enum Issue {
    BrokenImage {
        filename: PathBuf,
        line: u32,
        target: String,
    },
    BrokenLink {
        filename: PathBuf,
        line: u32,
        target: String,
    },
    DuplicateSection {
        filename: PathBuf,
        section_type: String,
    },
    EmptySection {
        filename: PathBuf,
        line: u32,
        section_type: String,
    },
    LinkToSameDocument {
        filename: PathBuf,
        line: u32,
    },
    LinkWithoutDestination {
        filename: PathBuf,
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
    ObsoleteLink {
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

pub struct MissingLink {
    pub path: PathBuf,
    pub title: String,
}

impl Display for Issue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Issue::BrokenImage {
                filename,
                line,
                target,
            } => write!(
                f,
                "{}:{}  image link to non-existing file \"{}\"",
                filename.to_string_lossy(),
                line,
                target
            ),
            Issue::BrokenLink {
                filename,
                line,
                target,
            } => write!(
                f,
                "{}:{}  link to non-existing file \"{}\"",
                filename.to_string_lossy(),
                line,
                target
            ),
            Issue::DuplicateSection {
                filename,
                section_type,
            } => write!(
                f,
                "{}  document contains multiple \"{}\" sections",
                filename.to_string_lossy(),
                section_type
            ),
            Issue::EmptySection {
                filename,
                line,
                section_type,
            } => write!(
                f,
                "{}:{}  section \"{}\" has no content",
                filename.to_string_lossy(),
                line + 1,
                section_type
            ),
            Issue::LinkToSameDocument { filename, line } => write!(
                f,
                "{}:{}  document contains link to itself",
                filename.to_string_lossy(),
                line
            ),
            Issue::LinkWithoutDestination { filename, line } => write!(
                f,
                "{}:{}  link without destination",
                filename.to_string_lossy(),
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
            Issue::ObsoleteLink { file, line } => write!(
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
