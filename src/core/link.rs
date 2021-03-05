use heck::KebabCase;

/// a link on a line
#[derive(Debug, PartialEq)]
pub struct Link {
  pub destination: String,
  pub title: String,
}

impl Link {
  /// provides the local anchor for this link
  pub fn local_anchor(&self) -> String {
    Link::to_anchor(&self.title)
  }

  /// converts the given link title into a GitHub-compatible link anchor
  fn to_anchor(title: &str) -> String {
    format!("#{}", title.to_kebab_case())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn to_anchor() {
    let tests = vec![("foo", "#foo")];
    for (give, want) in tests.into_iter() {
      assert_eq!(Link::to_anchor(give), want);
    }
  }
}
