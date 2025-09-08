/// provides the directory of the given file path
pub fn dirname(path: &str) -> &str {
  match path.rfind('/') {
    Some(pos) => &path[..pos],
    None => "",
  }
}

#[cfg(test)]
mod tests {

  mod dirname {

    #[test]
    fn normal() {
      let give = "one/two/file.md";
      let want = "one/two";
      let have = super::super::dirname(give);
      assert_eq!(have, want);
    }

    #[test]
    fn file_only() {
      let give = "file.md";
      let want = "";
      let have = super::super::dirname(give);
      assert_eq!(have, want);
    }

    #[test]
    fn dir_already() {
      let give = "one/two/";
      let want = "one/two";
      let have = super::super::dirname(give);
      assert_eq!(have, want);
    }
  }
}
