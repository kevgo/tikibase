use std::fmt::Display;

use camino::{Utf8Path, Utf8PathBuf};
use serde::Serialize;

use crate::database::paths;

/// a path relative to the root of the document base,
/// i.e. "foo.md"
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PathRelativeToRoot(Utf8PathBuf);

impl PathRelativeToRoot {
  pub fn as_str(&self) -> &str {
    self.0.as_str()
  }

  pub fn join(&self, fragment: impl AsRef<Utf8Path>) -> PathRelativeToRoot {
    return PathRelativeToRoot(self.0.join(fragment));
  }

  /// provides the lowest subdirectory portion of the given path
  /// If a subdir was found, removes it from the given path.
  pub fn lowest_subdir(&self) -> (&str, &str) {
    let self_str = self.as_str();
    match self_str.find('/') {
      Some(idx) => (&self_str[..idx], &self_str[idx + 1..]),
      None => ("", self_str),
    }
  }

  pub fn normalize(&self) -> PathRelativeToRoot {
    PathRelativeToRoot::from(paths::normalize(self.0.as_str()))
  }
}

impl AsRef<Utf8Path> for PathRelativeToRoot {
  fn as_ref(&self) -> &Utf8Path {
    self.0.as_ref()
  }
}

impl Display for PathRelativeToRoot {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.0.as_str())
  }
}

impl From<&PathRelativeToRoot> for PathRelativeToRoot {
  fn from(value: &PathRelativeToRoot) -> Self {
    PathRelativeToRoot(value.0.clone())
  }
}

impl From<&str> for PathRelativeToRoot {
  fn from(value: &str) -> Self {
    verify(value);
    PathRelativeToRoot(Utf8PathBuf::from(value))
  }
}

fn verify(path: &str) {
  if path.starts_with('/') {
    panic!("given an absolute path as a relative path");
  }
}
