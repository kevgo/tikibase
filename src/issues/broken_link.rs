use crate::config;
use crate::database::Tikibase;
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

    fn fix(&self, _base: &mut Tikibase, _config: &config::Data) -> String {
        panic!("not fixable")
    }

    fn fixable(&self) -> bool {
        false
    }
}
