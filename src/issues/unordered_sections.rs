use super::Problem;
use crate::fixers::unordered_sections::UnorderedSectionFixer;
use crate::fixers::Fix;
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;

/// describes the issue that a document has sections out of order
pub struct UnorderedSections {
    pub file: PathBuf,
}

impl Display for UnorderedSections {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}  wrong section order", self.file.to_string_lossy())
    }
}

impl Problem for UnorderedSections {
    fn fixer(self: Box<Self>) -> Option<Box<dyn Fix>> {
        Some(Box::new(UnorderedSectionFixer { issue: *self }))
    }
}
