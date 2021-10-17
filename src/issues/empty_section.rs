use crate::config;
use crate::database::Tikibase;
use crate::Issue;
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;

/// describes the issue that a section is empty
pub struct EmptySection {
    pub filename: PathBuf,
    pub line: u32,
    pub section_type: String,
}

impl Display for EmptySection {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}  section \"{}\" has no content",
            self.filename.to_string_lossy(),
            self.line + 1,
            self.section_type
        )
    }
}

impl Issue for EmptySection {
    fn fixable(&self) -> bool {
        true
    }

    fn fix(&self, base: &mut Tikibase, _config: &config::Data) -> String {
        let base_dir = &base.dir.clone();
        let doc = base.get_doc_mut(&self.filename).unwrap();
        doc.content_sections
            .retain(|section| section.section_type() != self.section_type);
        doc.save(base_dir.as_ref());
        format!(
            "{}:{}  removed empty section \"{}\"",
            self.filename.to_string_lossy(),
            self.line + 1,
            self.section_type
        )
    }
}
