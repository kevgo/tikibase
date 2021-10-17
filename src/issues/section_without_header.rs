use crate::Fix;
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;

pub struct SectionWithoutHeader {
    pub file: PathBuf,
    pub line: u32,
}

impl Display for SectionWithoutHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}  section has no title",
            self.file.to_string_lossy(),
            self.line + 1
        )
    }
}

impl Fix for SectionWithoutHeader {
    fn fixable(&self) -> bool {
        false
    }
}
