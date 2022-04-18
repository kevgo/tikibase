use super::{Directory, Document};
use crate::{Config, Issue};
use std::path::{Path, PathBuf};

pub struct Tikibase {
    pub root: PathBuf,
    pub dir: Directory,
}

impl Tikibase {
    pub fn load(root: PathBuf, config: &Config) -> Result<Tikibase, Vec<Issue>> {
        let dir = Directory::load(&root, config)?;
        Ok(Tikibase { root, dir })
    }

    pub fn get_doc<P: AsRef<Path>>(&self, relative_path: P) -> Option<&Document> {
        self.dir.get_doc(relative_path)
    }

    /// provides the document with the given relative filename as a mutable reference
    pub fn get_doc_mut<P: AsRef<Path>>(&mut self, path: P) -> Option<&mut Document> {
        self.dir.get_doc_mut(path)
    }
}
