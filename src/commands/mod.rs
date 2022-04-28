//! High-level commands that the Tikibase linter can execute.

mod check;
mod fix;
mod init;
mod json_schema;
mod pitstop;
mod stats;

// re-exports
pub use check::check;
pub use fix::fix;
pub use init::init;
pub use json_schema::json_schema;
pub use pitstop::pitstop;
pub use stats::stats;

// used locally
use crate::check::Issue;
use crate::Fix;

/// The inner API of the check subsystem.
/// This data structure is returned by the probes.
/// It contains highly structured, semantically meaningful data
/// intended to be used programmatically.
#[derive(Debug, Default, PartialEq)]
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
