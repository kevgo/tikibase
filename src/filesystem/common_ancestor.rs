/// provides the largest common ancestor for the two given paths
fn common_anchestor<'a>(path1: &'a str, path2: &str) -> &'a str {
  let mut path1_chars = path1.char_indices();
  let mut path2_chars = path2.chars();
  let mut slash_pos: usize = 0;
  loop {
    match (path1_chars.next(), path2_chars.next()) {
      (Some((pos, seg1)), Some(seg2)) if seg1 == seg2 => {
        if seg1 == '/' {
          slash_pos = pos;
        }
      }
      (None, None) => return path1,
      _ => return &path1[..slash_pos],
    }
  }
}

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
