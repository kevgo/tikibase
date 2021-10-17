use crate::Issue;
use std::path::PathBuf;

pub struct LinkToSameDocument {
    pub filename: PathBuf,
    pub line: u32,
}

impl Issue for LinkToSameDocument {
    fn describe(&self) -> String {
        format!(
            "{}:{}  link to the same file",
            self.filename.to_string_lossy(),
            self.line
        )
    }

    fn fixable(&self) -> bool {
        false
    }
}
