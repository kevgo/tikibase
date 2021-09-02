use crate::config;
use crate::database::Tikibase;
use crate::Issue;
use std::path::PathBuf;

pub struct LinkToSameDocument {
    pub filename: PathBuf,
    pub line: u32,
}

impl Issue for LinkToSameDocument {
    fn describe(&self) -> String {
        format!(
            "{}:{}  link to the same file",
            self.filename.to_string_lossy(),
            self.line
        )
    }

    fn fix(&self, _base: &mut Tikibase, _config: &config::Data) -> String {
        unimplemented!()
    }

    fn fixable(&self) -> bool {
        false
    }
}
