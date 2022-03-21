mod check;
mod fix;
mod pitstop;
mod stats;

use crate::fixers::Fix;
pub use check::check;
pub use fix::fix;
pub use pitstop::pitstop;
pub use stats::stats;
use std::path::PathBuf;

/// result of running a Tikibase command
#[derive(Default)]
pub struct Outcome {
    /// the issues identified but not fixed
    pub issues: Vec<Issue>,
    /// the fixes applied
    pub fixes: Vec<Fix>,
}

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
#[derive(Clone, Debug, PartialEq)]
pub struct MissingLink {
    pub path: PathBuf,
    pub title: String,
}
