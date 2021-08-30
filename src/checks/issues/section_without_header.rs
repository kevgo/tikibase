use crate::checks::Issue;
use crate::database::Tikibase;
use std::path::PathBuf;

pub struct SectionNoHeader {
    pub file: PathBuf,
    pub line: u32,
}

impl Issue for SectionNoHeader {
    fn describe(&self) -> String {
        format!(
            "{}:{}  section has no title",
            self.file.to_string_lossy(),
            self.line + 1
        )
    }

    fn fix(&self, _base: &mut Tikibase, _config: &crate::database::config::Data) -> String {
        panic!("not fixable");
    }

    fn fixable(&self) -> bool {
        false
    }
}
