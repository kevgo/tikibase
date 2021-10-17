use crate::Fix;
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

impl Fix for LinkToSameDocument {
    fn fixable(&self) -> bool {
        false
    }
}
