/// A non-Markdown file stored in a Tikibase.
pub struct Resource {
    /// the path of this file, relative to the Tikibase root
    pub path: std::path::PathBuf,
}
