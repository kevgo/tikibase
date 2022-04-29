use super::{Directory, Document};
use crate::check::Issue;
use crate::Config;

pub struct Tikibase {
    pub root: String,
    pub dir: Directory,
}

impl Tikibase {
    pub fn load(root: String) -> Result<Tikibase, Vec<Issue>> {
        let dir = Directory::load(&root, "".into(), Config::default())?;
        Ok(Tikibase { root, dir })
    }

    pub fn get_doc<P: AsRef<str>>(&self, relative_path: P) -> Option<&Document> {
        self.dir.get_doc(relative_path)
    }

    /// provides the document with the given relative filename as a mutable reference
    pub fn get_doc_mut<P: AsRef<str>>(&mut self, path: P) -> Option<&mut Document> {
        self.dir.get_doc_mut(path)
    }
}
