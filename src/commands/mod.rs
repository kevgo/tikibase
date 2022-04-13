//! High-level commands that the Tikibase linter can execute.

mod check;
mod fix;
mod init;
mod json_schema;
mod pitstop;
mod stats;

use crate::fix::Fix;
pub use check::check;
pub use fix::fix;
pub use init::init;
pub use json_schema::json_schema;
pub use pitstop::pitstop;
use serde::Serialize;
pub use stats::stats;
use std::path::PathBuf;

/// The inner API of Tikibase.
/// This data structure is returned by the probes.
/// It contains highly structured, semantically meaningful data
/// intended to be used programmatically.
#[derive(Default)]
pub struct Outcome {
    /// the issues identified but not fixed
    pub issues: Vec<Issue>,
    /// the fixes applied
    pub fixes: Vec<Fix>,
}

impl Outcome {
    /// provides an `Outcome` containing the given `Issue`
    pub fn from_issue(issue: Issue) -> Outcome {
        Outcome {
            issues: vec![issue],
            fixes: vec![],
        }
    }
}

/// the issues that this linter can find
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Issue {
    BrokenImage {
        location: Location,
        target: String,
    },
    CannotReadConfigurationFile {
        location: Location,
        message: String,
    },
    CannotWriteConfigFile {
        message: String,
        file: PathBuf,
    },
    CannotWriteJsonSchemaFile {
        file: PathBuf,
        message: String,
    },
    DocumentWithoutLinks {
        location: Location,
    },
    DuplicateSection {
        location: Location,
        title: String,
    },
    EmptyDocument {
        path: PathBuf,
    },
    EmptySection {
        location: Location,
        title: String,
    },
    InconsistentHeadingLevel {
        location: Location,
        /// human-readable section title
        section_title: String,
        /// the most commonly observed level (if one exists)
        common_variant: Option<u8>,
        /// the level used here
        this_variant: u8,
        /// all observed variants
        all_variants: Vec<u8>,
    },
    InvalidConfigurationFile {
        location: Location,
        message: String,
    },
    InvalidGlob {
        glob: String,
        message: String,
        location: Location,
    },
    InvalidTitleRegex {
        regex: String,
        problem: String,
        file: PathBuf,
    },
    LinkToNonExistingAnchorInCurrentDocument {
        location: Location,
        /// the non-existing anchor in the current
        anchor: String,
    },
    LinkToNonExistingAnchorInExistingDocument {
        location: Location,
        /// the file that the link points to
        target_file: String,
        /// the non-existing anchor in that file
        anchor: String,
    },
    LinkToNonExistingFile {
        location: Location,
        target: String,
    },
    LinkToSameDocument {
        location: Location,
    },
    LinkWithoutTarget {
        location: Location,
    },
    MissingFootnote {
        location: Location,
        identifier: String,
    },
    MissingLinks {
        location: Location,
        links: Vec<MissingLink>,
    },
    MixCapSection {
        location: Location,
        variants: Vec<String>,
    },
    NoTitleSection {
        location: Location,
    },
    ObsoleteOccurrencesSection {
        location: Location,
    },
    OrphanedResource {
        // This is a String and not a Path because we need a String (to print it),
        // and we already converted the Path of this orphaned resource into a String
        // during processing it.
        location: Location,
    },
    SectionWithoutHeader {
        location: Location,
    },
    TitleRegexNoCaptures {
        regex: String,
    },
    TitleRegexTooManyCaptures {
        regex: String,
        captures: usize,
    },
    UnclosedBacktick {
        location: Location,
    },
    UnclosedFence {
        location: Location,
    },
    UnknownSection {
        location: Location,
        title: String,
        allowed_titles: Vec<String>,
    },
    UnorderedSections {
        location: Location,
    },
    UnusedFootnote {
        location: Location,
        identifier: String,
    },
}

/// a missing link to a document
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MissingLink {
    pub path: PathBuf,
    pub title: String,
}

/// the position of an issue or fix
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialOrd, PartialEq, Serialize)]
pub struct Location {
    pub file: PathBuf,
    pub line: u32,
    pub start: u32,
    pub end: u32,
}
