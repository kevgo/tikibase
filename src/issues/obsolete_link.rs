use super::Problem;
use crate::fixers::obsolete_link::ObsoleteLinkFixer;
use crate::fixers::Fix;
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;

/// indicates that a document contains an "occurrences" section
/// that should no longer be there
pub struct ObsoleteLink {
    pub file: PathBuf,
    pub line: u32,
}

impl Display for ObsoleteLink {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}  obsolete occurrences section",
            self.file.to_string_lossy(),
            self.line + 1,
        )
    }
}

impl Problem for ObsoleteLink {
    fn fixer(self) -> Option<Box<dyn Fix>> {
        Some(Box::new(ObsoleteLinkFixer { issue: self }))
    }
}
