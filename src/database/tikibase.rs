use super::{Directory, Document};
use crate::check::Issue;
use crate::Config;
use std::ffi::OsStr;
use std::path::PathBuf;

pub struct Tikibase {
    pub root: PathBuf,
    pub dir: Directory,
}

impl Tikibase {
    pub fn load(root: PathBuf) -> Result<Tikibase, Vec<Issue>> {
        let dir = Directory::load(&root, PathBuf::from(""), Config::default())?;
        Ok(Tikibase { root, dir })
    }

    pub fn get_doc<P: AsRef<OsStr>>(&self, relative_path: P) -> Option<&Document> {
        self.dir.get_doc(relative_path.as_ref())
    }

    /// provides the document with the given relative filename as a mutable reference
    pub fn get_doc_mut<P: AsRef<OsStr>>(&mut self, path: P) -> Option<&mut Document> {
        self.dir.get_doc_mut(path)
    }
}

#[cfg(test)]
mod tests {

    use crate::database::Tikibase;
    use crate::test;

    #[test]
    fn subdirectories() {
        let dir = test::tmp_dir();
        test::create_file("sub1/one.md", "# test doc", &dir);
        let base = Tikibase::load(dir).unwrap();
        base.get_doc("sub1/one.md").unwrap();
    }
}
