use crate::Fix;
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;

pub struct LinkWithoutDestination {
    pub filename: PathBuf,
    pub line: u32,
}

impl Display for LinkWithoutDestination {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}  link without destination",
            self.filename.to_string_lossy(),
            self.line
        )
    }
}

impl Fix for LinkWithoutDestination {
    fn fixable(&self) -> bool {
        false
    }
}
