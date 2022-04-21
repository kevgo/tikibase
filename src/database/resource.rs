use std::path::PathBuf;

/// a non-markdown file stored in the Tikibase
pub struct Resource {
    /// the path of this file, relative to the Tikibase root
    // TODO: remove this struct
    pub path: PathBuf,
}
