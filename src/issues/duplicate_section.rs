use super::Problem;
use crate::fixers::Fix;
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;

/// describes the issue that a document contains two sections with the same title
pub struct DuplicateSection {
    pub filename: PathBuf,
    pub section_type: String,
}

impl Display for DuplicateSection {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}  duplicate section: {}",
            self.filename.to_string_lossy(),
            self.section_type
        )
    }
}

impl Problem for DuplicateSection {
    fn fixer(self: Box<Self>) -> Option<Box<dyn Fix>> {
        None
    }
}
