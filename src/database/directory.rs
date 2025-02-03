use super::{paths, Document};
use crate::check::Issue;
use crate::config::LoadResult;
use crate::{config, Config};
use ahash::HashMap;
use fs_err as fs;
use merge::Merge;

pub struct Directory {
  pub relative_path: String,
  pub config: Config,
  pub dirs: HashMap<String, Directory>,
  pub docs: HashMap<String, Document>,
  pub resources: HashMap<String, ()>,
}

impl Directory {
  /// provides the directory with the given relative filename
  pub fn get_dir(&self, relative_path: &str) -> Option<&Self> {
    match lowest_subdir(relative_path) {
      ("", filename) => self.dirs.get(filename),
      (subdir, remaining_path) => match self.dirs.get(subdir) {
        Some(dir) => dir.get_dir(remaining_path),
        None => None,
      },
    }
  }

  /// provides the document with the given relative filename
  pub fn get_doc(&self, relative_path: &str) -> Option<&Document> {
    match lowest_subdir(relative_path) {
      ("", filename) => self.docs.get(filename),
      (subdir, remaining_path) => match self.dirs.get(subdir) {
        Some(dir) => dir.get_doc(remaining_path),
        None => None,
      },
    }
  }

  /// provides the document with the given relative filename as a mutable reference
  pub fn get_doc_mut(&mut self, relative_path: &str) -> Option<&mut Document> {
    match lowest_subdir(relative_path) {
      ("", filename) => self.docs.get_mut(filename),
      (subdir, remaining_path) => match self.dirs.get_mut(subdir) {
        Some(dir) => dir.get_doc_mut(remaining_path),
        None => None,
      },
    }
  }

  /// indicates whether this Tikibase contains a directory with the given name
  pub fn has_dir(&self, path: &str) -> bool {
    match lowest_subdir(path) {
      ("", filename) => self.dirs.contains_key(filename),
      (subdir, remaining_path) => match self.dirs.get(subdir) {
        Some(dir) => dir.has_dir(remaining_path),
        None => false,
      },
    }
  }

  /// indicates whether this Tikibase contains a resource with the given path
  pub fn has_resource(&self, path: &str) -> bool {
    match lowest_subdir(path) {
      ("", filename) => self.resources.contains_key(filename),
      (subdir, remaining_path) => match self.dirs.get(subdir) {
        Some(dir) => dir.has_resource(remaining_path),
        None => false,
      },
    }
  }

  /// provides a Directory instance for the given directory
  pub fn load(
    root: &str,
    relative_path: String,
    mut parent_config: Config,
  ) -> Result<Self, Vec<Issue>> {
    let abs_path = paths::join(root, &relative_path);
    let config = match config::load(&abs_path) {
      LoadResult::Loaded(config) => {
        parent_config.merge(config);
        parent_config
      }
      LoadResult::NotFound => parent_config,
      LoadResult::Error(issue) => return Err(vec![issue]),
    };
    let mut docs = HashMap::new();
    let mut dirs = HashMap::new();
    let mut resources = HashMap::new();
    let mut errors = Vec::new();
    let entries = match fs::read_dir(&abs_path) {
      Ok(entries) => entries,
      Err(err) => {
        return Err(vec![Issue::CannotReadDirectory {
          path: abs_path,
          err: err.to_string(),
        }])
      }
    };
    for entry in entries {
      let entry = entry.unwrap();
      let entry_name = entry.file_name().to_string_lossy().to_string();
      let entry_abs_path = entry.path();
      match EntryType::from_direntry(&entry, &config) {
        EntryType::Document => {
          let doc_relative_path = paths::join(&relative_path, &entry_name);
          match Document::load(&entry_abs_path, doc_relative_path) {
            Ok(doc) => {
              docs.insert(entry_name, doc);
            }
            Err(err) => errors.push(err),
          }
        }
        EntryType::Resource => {
          resources.insert(entry_name, ());
        }
        EntryType::Configuration | EntryType::Ignored => continue,
        EntryType::Directory => {
          dirs.insert(
            entry_name.clone(),
            Self::load(
              root,
              paths::join(&relative_path, &entry_name),
              config.clone(),
            )?,
          );
        }
      }
    }
    if errors.is_empty() {
      Ok(Self {
        relative_path,
        config,
        dirs,
        docs,
        resources,
      })
    } else {
      Err(errors)
    }
  }
}

/// filesystem entry types that Tikibase distinguishes
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EntryType {
  /// subdirectory of the current directory
  Directory,
  /// Markdown document
  Document,
  /// linkable resource
  Resource,
  /// Tikibase configuration file
  Configuration,
  /// ignored file
  Ignored,
}

impl EntryType {
  fn from_direntry(entry: &fs::DirEntry, config: &Config) -> Self {
    let entry_type = entry.file_type().unwrap();
    let entry_filename_os = entry.file_name();
    let entry_filename = entry_filename_os.to_string_lossy();
    if entry_filename.starts_with('.') {
      return Self::Ignored;
    }
    if entry_type.is_file() {
      if entry_filename == "tikibase.json" {
        return Self::Configuration;
      }
      if config.ignore(&entry_filename) {
        return Self::Ignored;
      }
      if has_extension(&entry_filename, "md") {
        return Self::Document {};
      }
      return Self::Resource;
    }
    if entry_type.is_dir() {
      return Self::Directory;
    }
    Self::Ignored
  }

  pub fn from_str(path: &str) -> Self {
    if path == "tikibase.json" {
      return Self::Configuration;
    }
    if path.starts_with('.') {
      return Self::Ignored;
    }
    if has_extension(path, "md") {
      return Self::Document;
    }
    if path.ends_with('/') {
      return Self::Directory;
    }
    Self::Resource
  }
}

/// case-insensitive comparison of file extensions
fn has_extension(path: &str, given_ext: &str) -> bool {
  let path_ext = path.rsplit('.').next().unwrap();
  path_ext.eq_ignore_ascii_case(given_ext)
}

/// provides the lowest subdirectory portion of the given path
/// If a subdir was found, removes it from the given path.
fn lowest_subdir(path: &str) -> (&str, &str) {
  match path.find('/') {
    Some(idx) => (&path[..idx], &path[idx + 1..]),
    None => ("", path),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test;
  use big_s::S;
  use indoc::indoc;

  #[test]
  fn empty() {
    let dir = test::tmp_dir();
    let dir = Directory::load(&dir, S(""), Config::default()).unwrap();
    assert_eq!(dir.docs.len(), 0);
    assert_eq!(dir.resources.len(), 0);
  }

  mod entry_type {
    use crate::database::EntryType;

    #[test]
    fn from_str() {
      let tests = vec![
        ("foo.md", EntryType::Document),
        ("sub/foo.md", EntryType::Document),
        ("foo.png", EntryType::Resource),
        ("foo.pdf", EntryType::Resource),
        (".testconfig.json", EntryType::Ignored),
        ("dir/", EntryType::Directory),
      ];
      for (give, want) in tests {
        let have = EntryType::from_str(give);
        assert_eq!(have, want);
      }
    }
  }

  mod get_dir {
    use crate::database::Directory;
    use crate::{test, Config};
    use big_s::S;

    #[test]
    fn exists() {
      let dir = test::tmp_dir();
      test::create_file("one/two/one.md", "# test doc", &dir);
      let root = Directory::load(&dir, S(""), Config::default()).unwrap();
      let have = root.get_dir("one/two").unwrap();
      assert_eq!(have.relative_path, "one/two");
    }

    #[test]
    fn missing() {
      let dir = test::tmp_dir();
      let dir = Directory::load(&dir, S(""), Config::default()).unwrap();
      assert!(dir.get_dir("zonk").is_none());
    }
  }

  mod get_doc {
    use crate::database::Directory;
    use crate::{test, Config};
    use big_s::S;

    #[test]
    fn exists() {
      let dir = test::tmp_dir();
      test::create_file("one.md", "# test doc", &dir);
      let dir = Directory::load(&dir, S(""), Config::default()).unwrap();
      let doc = dir.get_doc("one.md").unwrap();
      assert_eq!(doc.title_section.title_line.text, "# test doc");
    }

    #[test]
    fn missing() {
      let dir = test::tmp_dir();
      let dir = Directory::load(&dir, S(""), Config::default()).unwrap();
      assert!(dir.get_doc("zonk.md").is_none());
    }
  }

  mod get_doc_mut {
    use crate::database::Directory;
    use crate::{test, Config};
    use big_s::S;

    #[test]
    fn exists() {
      let dir = test::tmp_dir();
      test::create_file("one.md", "# test doc", &dir);
      let mut dir = Directory::load(&dir, S(""), Config::default()).unwrap();
      let doc = dir.get_doc_mut("one.md").unwrap();
      assert_eq!(doc.title_section.title_line.text, "# test doc");
    }

    #[test]
    fn missing() {
      let dir = test::tmp_dir();
      let mut dir = Directory::load(&dir, S(""), Config::default()).unwrap();
      assert!(dir.get_doc_mut("zonk.md").is_none());
    }
  }

  #[test]
  fn has_extension() {
    let tests = vec![
      (("foo.md", "md"), true),
      (("FOO.MD", "md"), true),
      (("foo.md", "MD"), true),
      (("foo.md", "png"), false),
    ];
    for (give, want) in tests {
      let have = super::has_extension(give.0, give.1);
      assert_eq!(have, want);
    }
  }

  mod has_resource {
    use crate::database::Directory;
    use crate::{test, Config};
    use big_s::S;

    #[test]
    fn mismatch() {
      let dir = test::tmp_dir();
      let dir = Directory::load(&dir, S(""), Config::default()).unwrap();
      assert!(!dir.has_resource("zonk.png"));
    }

    #[test]
    fn matching() {
      let root = test::tmp_dir();
      test::create_file("one/two/foo.png", "content", &root);
      let dir = Directory::load(&root, S(""), Config::default()).unwrap();
      assert!(dir.has_resource("one/two/foo.png"));
    }
  }

  #[test]
  fn load() {
    let dir = test::tmp_dir();
    let content = indoc! {"
            # Title
            title text
            ### Section 1
            one
            two
            ### Section 2
            foo
            "};
    test::create_file("file.md", content, &dir);
    let dir = Directory::load(&dir, S(""), Config::default()).unwrap();
    // make sure we can load existing documents
    let _doc = &dir.get_doc("file.md").unwrap();
  }

  #[test]
  fn load_hidden_file() {
    let dir = test::tmp_dir();
    test::create_file(".hidden", "content", &dir);
    let dir = Directory::load(&dir, S(""), Config::default()).unwrap();
    assert_eq!(dir.resources.len(), 0);
  }

  mod lowest_subdir {

    #[test]
    fn top_level() {
      let give = "foo.md";
      let want = ("", "foo.md");
      let have = super::super::lowest_subdir(give);
      assert_eq!(have, want);
    }

    #[test]
    fn subdir() {
      let give = "sub1/foo.md";
      let want = ("sub1", "foo.md");
      let have = super::super::lowest_subdir(give);
      assert_eq!(have, want);
    }

    #[test]
    fn nested_subdir() {
      let give = "sub1/sub2/foo.md";
      let want = ("sub1", "sub2/foo.md");
      let have = super::super::lowest_subdir(give);
      assert_eq!(have, want);
    }
  }
}
