use crate::Issue;
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;

/// describes a broken link in the Tikibase
pub struct BrokenLink {
    pub filename: PathBuf,
    pub line: u32,
    pub target: String,
}

impl Display for BrokenLink {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}  broken link to \"{}\"",
            self.filename.to_string_lossy(),
            self.line,
            self.target
        )
    }
}

impl Issue for BrokenLink {
    fn fixable(&self) -> bool {
        false
    }
}
