use crate::Issue;
use std::path::PathBuf;

pub struct SectionWithoutHeader {
    pub file: PathBuf,
    pub line: u32,
}

impl Issue for SectionWithoutHeader {
    fn describe(&self) -> String {
        format!(
            "{}:{}  section has no title",
            self.file.to_string_lossy(),
            self.line + 1
        )
    }

    fn fixable(&self) -> bool {
        false
    }
}
