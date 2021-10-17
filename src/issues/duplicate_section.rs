use crate::Issue;
use std::path::PathBuf;

/// describes the issue that a document contains two sections with the same title
pub struct DuplicateSection {
    pub filename: PathBuf,
    pub section_type: String,
}

impl Issue for DuplicateSection {
    fn fixable(&self) -> bool {
        false
    }

    fn describe(&self) -> String {
        format!(
            "{}  duplicate section: {}",
            self.filename.to_string_lossy(),
            self.section_type
        )
    }
}
