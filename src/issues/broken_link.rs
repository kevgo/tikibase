use crate::Issue;
use std::path::PathBuf;

/// describes a broken link in the Tikibase
pub struct BrokenLink {
    pub filename: PathBuf,
    pub line: u32,
    pub target: String,
}

impl Issue for BrokenLink {
    fn describe(&self) -> String {
        format!(
            "{}:{}  broken link to \"{}\"",
            self.filename.to_string_lossy(),
            self.line,
            self.target
        )
    }

    fn fixable(&self) -> bool {
        false
    }
}
