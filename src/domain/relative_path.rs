use camino::Utf8PathBuf;

/// a path relative to the root of the document base,
/// i.e. "foo.md"
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PathRelativeToRoot(Utf8PathBuf);

fn verify(path: Utf8PathBuf) {
  if path.as_str().starts_with('/') {
    panic!("given an absolute path as a relative path");
  }
}
