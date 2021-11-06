use super::Problem;
use crate::fixers::empty_section::EmptySectionFixer;
use crate::fixers::Fix;
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;

/// an empty section that was found in a document
pub struct EmptySection {
    pub filename: PathBuf,
    pub line: u32,
    pub section_type: String,
}

impl Display for EmptySection {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}  section \"{}\" has no content",
            self.filename.to_string_lossy(),
            self.line + 1,
            self.section_type
        )
    }
}

impl Problem for EmptySection {
    fn fixer(&self) -> Option<Box<dyn Fix + '_>> {
        Some(Box::new(EmptySectionFixer { issue: self }))
    }
}
