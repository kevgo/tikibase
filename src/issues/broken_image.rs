use super::Problem;
use crate::fixers::Fix;
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;

/// describes a broken image in the Tikibase
pub struct BrokenImage {
    pub filename: PathBuf,
    pub line: u32,
    pub target: String,
}

impl Display for BrokenImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}  broken image \"{}\"",
            self.filename.to_string_lossy(),
            self.line,
            self.target
        )
    }
}

impl Problem for BrokenImage {
    fn fixer(self: Box<Self>) -> Option<Box<dyn Fix>> {
        None
    }
}
