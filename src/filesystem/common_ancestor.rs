#[cfg(test)]
mod tests {

  mod common_anchestor {

    #[test]
    fn has_common_ancestors() {
      let path1 = "one/two/three/file.md";
      let path2 = "one/two/throw/file.md";
      let want = "one/two";
      let have = super::super::common_anchestor(path1, path2);
      assert_eq!(have, want);
    }

    #[test]
    fn no_common_ancestors() {
      let path1 = "one/two/three/file.md";
      let path2 = "alpha/beta/file.md";
      let want = "";
      let have = super::super::common_anchestor(path1, path2);
      assert_eq!(have, want);
    }

    #[test]
    fn identical() {
      let path1 = "one/two/three/file.md";
      let path2 = "one/two/three/file.md";
      let want = "one/two/three/file.md";
      let have = super::super::common_anchestor(path1, path2);
      assert_eq!(have, want);
    }
  }
}
