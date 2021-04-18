use super::document::Document;
use super::persistence;
use std::path::{Path, PathBuf};

pub struct Tikibase {
    pub dir: PathBuf,
    pub docs: Vec<Document>,
    pub resources: Vec<Resource>,
}

impl Tikibase {
    /// creates a new document with the given content in this Tikibase
    pub fn create_doc(&mut self, filename: &Path, content: &str) {
        let filepath = self.dir.join(filename);
        persistence::save_file(&filepath, content);
        self.docs.push(Document::from_str(filepath, content));
    }
}

/// A non-Markdown file stored in a Tikibase.
pub struct Resource {
    pub path: std::path::PathBuf,
}
