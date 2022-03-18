use std::borrow::Cow;
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;

/// possible issues that this linter can find
pub enum Issue {
    /// image link to a non-existing file
    BrokenImage {
        filename: PathBuf,
        line: u32,
        target: String,
    },
    /// link to a non-existing file
    BrokenLink {
        filename: PathBuf,
        line: u32,
        target: String,
    },
    /// a document contains two sections with the same title
    DuplicateSection {
        filename: PathBuf,
        section_type: String,
    },
    /// a section has no content
    EmptySection {
        filename: PathBuf,
        line: u32,
        section_type: String,
    },
    /// a document contains a link to itself
    LinkToSameDocument { filename: PathBuf, line: u32 },
    /// a link contains no target
    LinkWithoutDestination { filename: PathBuf, line: u32 },
    /// the "occurrences" section of the document is missing these links
    MissingLinks {
        file: PathBuf,
        links: Vec<MissingLink>,
    },
    /// a document references a source that doesn't exist
    MissingSource {
        file: String,
        line: u32,
        index: String,
    },
    /// a section title occurs with inconsistent capitalizations
    MixCapSection { variants: Vec<String> },
    /// a document contains an "occurrences" section that should no longer be there
    ObsoleteLink { file: PathBuf, line: u32 },
    /// a file that isn't linked to
    OrphanedResource {
        // This is a String and not a Path because we need a String (to print it),
        // and we already converted the Path of this orphaned resource into a String
        // during processing it.
        path: String,
    },
    /// a section whose title is empty
    SectionWithoutHeader { file: PathBuf, line: u32 },
    /// a section that isn't listed in tikibase.json
    UnknownSection {
        file: PathBuf,
        line: u32,
        section_type: String,
        allowed_types: Vec<String>,
    },
    /// a document contains sections in a different order than specified in tikibase.json
    UnorderedSections { file: PathBuf },
}

struct MissingLink {
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
                "{}:{}  broken image \"{}\"",
                self.filename.to_string_lossy(),
                self.line,
                self.target
            ),
            Issue::BrokenLink {
                filename,
                line,
                target,
            } => write!(
                f,
                "{}:{}  broken link to \"{}\"",
                self.filename.to_string_lossy(),
                self.line,
                self.target
            ),
            Issue::DuplicateSection {
                filename,
                section_type,
            } => write!(
                f,
                "{}  duplicate section: {}",
                self.filename.to_string_lossy(),
                self.section_type
            ),
            Issue::EmptySection {
                filename,
                line,
                section_type,
            } => write!(
                f,
                "{}:{}  section \"{}\" has no content",
                self.filename.to_string_lossy(),
                self.line + 1,
                self.section_type
            ),
            Issue::LinkToSameDocument { filename, line } => write!(
                f,
                "{}:{}  link to the same file",
                self.filename.to_string_lossy(),
                self.line
            ),
            Issue::LinkWithoutDestination { filename, line } => write!(
                f,
                "{}:{}  link without destination",
                self.filename.to_string_lossy(),
                self.line
            ),
            Issue::MissingLinks { file, links } => {
                let links: Vec<Cow<str>> = self
                    .links
                    .iter()
                    .map(|ml| ml.path.to_string_lossy())
                    .collect();
                write!(
                    f,
                    "{}  missing link to {}",
                    self.file.to_string_lossy(),
                    links.join(", "),
                )
            }
            Issue::MissingSource { file, line, index } => write!(
                f,
                "{}:{}  missing source [{}]",
                self.file,
                self.line + 1,
                self.index
            ),
            Issue::MixCapSection { variants } => write!(
                f,
                "mixed capitalization of sections: {}",
                self.variants.join("|")
            ),
            Issue::ObsoleteLink { file, line } => write!(
                f,
                "{}:{}  obsolete occurrences section",
                self.file.to_string_lossy(),
                self.line + 1,
            ),
            Issue::OrphanedResource { path } => write!(f, "unused resource \"{}\"", self.path),
            Issue::SectionWithoutHeader { file, line } => write!(
                f,
                "{}:{}  section has no title",
                self.file.to_string_lossy(),
                self.line + 1
            ),
            Issue::UnknownSection {
                file,
                line,
                section_type,
                allowed_types,
            } => {
                let alloweds: Vec<String> = self
                    .allowed_types
                    .iter()
                    .map(|allowed| format!("\n  - {}", allowed))
                    .collect();
                write!(
                    f,
                    "{}:{}  unknown section \"{}\", allowed sections:{}",
                    self.file.to_string_lossy(),
                    self.line + 1,
                    self.section_type,
                    alloweds.join("")
                )
            }
            Issue::UnorderedSections { file } => {
                write!(f, "{}  wrong section order", self.file.to_string_lossy())
            }
        }
    }
}
