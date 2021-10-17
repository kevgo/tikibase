use crate::config;
use crate::database::Tikibase;
use crate::Issue;
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

impl Issue for ObsoleteLink {
    fn fix(&self, base: &mut Tikibase, _config: &config::Data) -> String {
        let base_dir = base.dir.clone();
        let doc = base.get_doc_mut(&self.file).unwrap();
        // we can simply flush the document here because
        // its "occurrences" section was filtered out when loading the document
        doc.save(&base_dir);
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
