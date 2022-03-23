//! High-level commands that the Tikibase linter can execute.

mod check;
mod fix;
mod pitstop;
mod stats;

use crate::fix::Fix;
pub use check::check;
pub use fix::fix;
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

/// the issues that this linter can find
#[derive(Clone, Debug, PartialEq)]
pub enum Issue {
    BrokenImage {
        location: Location,
        target: String,
    },
    BrokenLink {
        location: Location,
        target: String,
    },
    CannotReadConfigurationFile {
        location: Location,
        message: String,
    },
    DuplicateSection {
        location: Location,
        title: String,
    },
    EmptySection {
        location: Location,
        title: String,
    },
    InvalidConfigurationFile {
        location: Location,
        message: String,
    },
    LinkToSameDocument {
        location: Location,
    },
    LinkWithoutDestination {
        location: Location,
    },
    MissingLinks {
        location: Location,
        links: Vec<MissingLink>,
    },
    MissingSource {
        location: Location,
        identifier: String,
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
}

/// a missing link to a document
#[derive(Clone, Debug, PartialEq)]
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
