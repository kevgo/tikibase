use crate::Issue;
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

impl Issue for BrokenImage {
    fn fixable(&self) -> bool {
        false
    }
}
