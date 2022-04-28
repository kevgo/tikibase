use serde::Serialize;
use std::path::PathBuf;

/// the position of an issue or fix within a file
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialOrd, PartialEq, Serialize)]
pub struct Location {
    pub file: PathBuf,
    pub line: u32,
    pub start: u32,
    pub end: u32,
}
