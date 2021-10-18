use std::borrow::Cow;
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;

pub struct MissingLink {
    pub path: PathBuf,
    pub title: String,
}

/// missing links in a document
pub struct MissingLinks {
    pub file: PathBuf,
    pub links: Vec<MissingLink>,
}

impl Display for MissingLinks {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let links: Vec<Cow<str>> = self
            .links
            .iter()
            .map(|ml| ml.path.to_string_lossy())
            .collect();
        write!(
            f,
            "{}  missing link to {}",
            self.file.to_string_lossy(),
            links.join(", "),
        )
    }
}
