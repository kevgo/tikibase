use super::{Directory, Document};
use crate::Config;
use crate::check::Issue;
use big_s::S;
use camino::Utf8PathBuf;

pub struct Tikibase {
  pub root: Utf8PathBuf,
  pub dir: Directory,
}

impl Tikibase {
  pub fn load<P: Into<Utf8PathBuf>>(root: P) -> Result<Self, Vec<Issue>> {
    let root = root.into();
    let dir = Directory::load(&root, S(""), Config::default())?;
    Ok(Self { root, dir })
  }

  pub fn get_dir(&self, relative_path: &str) -> Option<&Directory> {
    self.dir.get_dir(relative_path)
  }

  pub fn get_doc(&self, relative_path: &str) -> Option<&Document> {
    self.dir.get_doc(relative_path)
  }

  /// provides the document with the given relative filename as a mutable reference
  pub fn get_doc_mut(&mut self, path: &str) -> Option<&mut Document> {
    self.dir.get_doc_mut(path)
  }
}

#[cfg(test)]
mod tests {

  mod get_doc {
    use crate::database::Tikibase;
    use crate::test;

    #[test]
    fn subdirectory() {
      let dir = camino_tempfile::tempdir().unwrap();
      test::create_file("sub1/one.md", "# test doc", dir.path());
      let base = Tikibase::load(dir.path()).unwrap();
      base.get_doc("sub1/one.md").unwrap();
    }
  }
}
