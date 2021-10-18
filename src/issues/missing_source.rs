use super::Problem;
use crate::fixers::Fix;
use std::fmt::{self, Display, Formatter};

pub struct MissingSource {
    pub file: String,
    pub line: u32,
    pub index: String,
}

impl Display for MissingSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}  missing source [{}]",
            self.file,
            self.line + 1,
            self.index
        )
    }
}

impl Problem for MissingSource {
    fn fixer(self) -> Option<Box<dyn Fix>> {
        None
    }
}
