use crate::database::{config, Tikibase};
use crate::Issue;
use std::path::PathBuf;

/// describes a broken image in the Tikibase
pub struct BrokenImage {
    pub filename: PathBuf,
    pub line: u32,
    pub target: String,
}

impl Issue for BrokenImage {
    fn describe(&self) -> String {
        format!(
            "{}:{}  broken image \"{}\"",
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
