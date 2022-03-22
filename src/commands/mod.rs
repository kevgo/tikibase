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
        pos: Position,
        target: String,
    },
    BrokenLink {
        pos: Position,
        target: String,
    },
    CannotReadConfigurationFile {
        message: String,
        pos: Position,
    },
    DuplicateSection {
        pos: Position,
        section_type: String,
    },
    EmptySection {
        pos: Position,
        section_type: String,
    },
    InvalidConfigurationFile {
        message: String,
        pos: Position,
    },
    LinkToSameDocument {
        pos: Position,
    },
    LinkWithoutDestination {
        pos: Position,
    },
    MissingLinks {
        pos: Position,
        links: Vec<MissingLink>,
    },
    MissingSource {
        pos: Position,
        index: String,
    },
    MixCapSection {
        pos: Position,
        variants: Vec<String>,
    },
    NoTitleSection {
        pos: Position,
    },
    ObsoleteOccurrencesSection {
        pos: Position,
    },
    OrphanedResource {
        // This is a String and not a Path because we need a String (to print it),
        // and we already converted the Path of this orphaned resource into a String
        // during processing it.
        pos: Position,
    },
    SectionWithoutHeader {
        pos: Position,
    },
    UnclosedFence {
        pos: Position,
    },
    UnknownSection {
        pos: Position,
        section_type: String,
        allowed_types: Vec<String>,
    },
    UnorderedSections {
        pos: Position,
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
pub struct Position {
    pub file: PathBuf,
    pub line: u32,
    // pub start: u32,
    // pub end: u32,
}
