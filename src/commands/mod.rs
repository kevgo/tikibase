//! High-level commands that the Tikibase linter can execute.

mod check;
mod fix;
mod init;
mod json_schema;
mod pitstop;
mod stats;

use crate::check::Issue;
use crate::Fix;
pub use check::check;
pub use fix::fix;
pub use init::init;
pub use json_schema::json_schema;
pub use pitstop::pitstop;
pub use stats::stats;

/// The inner API of the check subsystem.
/// This data structure is returned by the probes.
/// It contains highly structured, semantically meaningful data
/// intended to be used programmatically.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct Outcome {
  /// the issues identified but not fixed
  pub issues: Vec<Issue>,
  /// the fixes applied
  pub fixes: Vec<Fix>,
}

impl Outcome {
  /// provides an `Outcome` containing the given `Issue`
  #[must_use]
  pub fn from_issue(issue: Issue) -> Self {
    Self {
      issues: vec![issue],
      fixes: vec![],
    }
  }
}
