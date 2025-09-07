use serde::Serialize;

use crate::domain::PathRelativeToRoot;

/// the position of an issue or fix within a file
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialOrd, PartialEq, Serialize)]
pub struct Location {
  pub file: PathRelativeToRoot,
  pub line: u32,
  pub start: u32,
  pub end: u32,
}
