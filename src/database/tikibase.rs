use super::{Directory, Document};
use crate::{Config, Issue};
use std::ffi::OsStr;
use std::path::PathBuf;

pub struct Tikibase {
    pub root: PathBuf,
    pub dir: Directory,
}

impl Tikibase {
    pub fn load(root: PathBuf) -> Result<Tikibase, Vec<Issue>> {
        let dir = Directory::load(&root, Config::default())?;
        Ok(Tikibase { root, dir })
    }

    pub fn get_doc<P: AsRef<OsStr>>(&self, relative_path: P) -> Option<&Document> {
        self.dir.get_doc(relative_path)
    }

    /// provides the document with the given relative filename as a mutable reference
    pub fn get_doc_mut<P: AsRef<OsStr>>(&mut self, path: P) -> Option<&mut Document> {
        self.dir.get_doc_mut(path)
    }
}
