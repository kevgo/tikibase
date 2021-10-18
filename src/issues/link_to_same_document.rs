use super::Problem;
use crate::fixers::Fix;
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;

pub struct LinkToSameDocument {
    pub filename: PathBuf,
    pub line: u32,
}

impl Display for LinkToSameDocument {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}  link to the same file",
            self.filename.to_string_lossy(),
            self.line
        )
    }
}

impl Problem for LinkToSameDocument {
    fn fixer(self: Box<Self>) -> Option<Box<dyn Fix>> {
        None
    }
}
