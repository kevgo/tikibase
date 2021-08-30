use crate::database::{config, Tikibase};
use crate::Issue;
use std::path::PathBuf;

/// indicates that a document contains an "occurrences" section
/// that should no longer be there
pub struct ObsoleteLink {
    pub file: PathBuf,
    pub line: u32,
}

impl Issue for ObsoleteLink {
    fn describe(&self) -> String {
        format!(
            "{}:{}  obsolete occurrences section",
            self.file.to_string_lossy(),
            self.line + 1,
        )
    }

    fn fix(&self, base: &mut Tikibase, _config: &config::Data) -> String {
        let base_dir = base.dir.clone();
        let doc = base.get_doc_mut(&self.file).unwrap();
        // we can simply flush the document here because
        // its "occurrences" section was filtered out when loading the document
        doc.flush(&base_dir);
        format!(
            "{}:{}  removed obsolete occurrences section",
            self.file.to_string_lossy(),
            self.line + 1,
        )
    }

    fn fixable(&self) -> bool {
        true
    }
}
