use super::{Directory, Document};
use crate::check::Issue;
use crate::Config;
use big_s::S;

pub struct Tikibase {
  pub root: String,
  pub dir: Directory,
}

impl Tikibase {
  pub fn load(root: String) -> Result<Self, Vec<Issue>> {
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
      let dir = test::tmp_dir();
      test::create_file("sub1/one.md", "# test doc", &dir);
      let base = Tikibase::load(dir).unwrap();
      base.get_doc("sub1/one.md").unwrap();
    }
  }
}
