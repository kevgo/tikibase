use crate::config;
use crate::database::Tikibase;
use crate::Issue;
use std::path::PathBuf;

pub struct LinkWithoutDestination {
    pub filename: PathBuf,
    pub line: u32,
}

impl Issue for LinkWithoutDestination {
    fn describe(&self) -> String {
        format!(
            "{}:{}  link without destination",
            self.filename.to_string_lossy(),
            self.line
        )
    }

    fn fix(&self, _base: &mut Tikibase, _config: &config::Data) -> String {
        panic!("not fixable");
    }

    fn fixable(&self) -> bool {
        false
    }
}
