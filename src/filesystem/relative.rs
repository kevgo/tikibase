/// provides the relative path from within given source file to the given target file
pub fn relative(source: &str, target: &str) -> String {
  let common_ancestor = common_anchestor(source, target);
  let source_ups = dirs_between(dirname(source), common_ancestor.len());
  let target_part = path_after(target, common_ancestor.len());
  format!("{}{}", go_up(source_ups), target_part)
}

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

/// provides the number of path segments between the given position in the given path and its end
pub fn dirs_between(path: &str, start: usize) -> usize {
  if start == 0 {
    if path.is_empty() {
      return 0;
    }
    return path.matches('/').count() + 1;
  }
  path[start..].matches('/').count()
}

fn go_up(count: usize) -> String {
  "../".repeat(count)
}

/// provides the part of the given path after the given prefix
fn path_after(path: &str, pos: usize) -> &str {
  match pos {
    0 => path,
    len if len == path.len() => "",
    len => &path[len + 1..],
  }
}

#[cfg(test)]
mod tests {

  mod dirs_between {

    #[test]
    fn normal() {
      let text = "one/two/three/four/five";
      let have = super::super::dirs_between(text, 7);
      let want = 3;
      assert_eq!(have, want);
    }

    #[test]
    fn full() {
      let text = "one/two/three/four/five";
      let have = super::super::dirs_between(text, 0);
      let want = 5;
      assert_eq!(have, want);
    }

    #[test]
    fn none() {
      let text = "one/two/three/four/five";
      let have = super::super::dirs_between(text, 23);
      let want = 0;
      assert_eq!(have, want);
    }

    #[test]
    fn empty() {
      let have = super::super::dirs_between("", 0);
      let want = 0;
      assert_eq!(have, want);
    }
  }

  mod go_up {
    use big_s::S;

    #[test]
    fn zero() {
      let have = super::super::go_up(0);
      let want = String::new();
      assert_eq!(have, want);
    }

    #[test]
    fn some() {
      let have = super::super::go_up(3);
      let want = S("../../../");
      assert_eq!(have, want);
    }
  }

  mod path_after {

    #[test]
    fn none() {
      let path = "one/two/three/four/five";
      let ancestor = "";
      let have = super::super::path_after(path, ancestor.len());
      let want = "one/two/three/four/five";
      assert_eq!(have, want);
    }

    #[test]
    fn some() {
      let path = "one/two/three/four/five";
      let ancestor = "one/two";
      let have = super::super::path_after(path, ancestor.len());
      let want = "three/four/five";
      assert_eq!(have, want);
    }

    #[test]
    fn full() {
      let path = "one/two/three/four/five";
      let ancestor = "one/two/three/four/five";
      let have = super::super::path_after(path, ancestor.len());
      let want = "";
      assert_eq!(have, want);
    }
  }

  mod relative {

    #[test]
    fn has_common_ancestors() {
      let path1 = "one/two/three/file.md";
      let path2 = "one/two/tralala/file.md";
      let have = super::super::relative(path1, path2);
      let want = "../tralala/file.md";
      assert_eq!(have, want);
    }

    #[test]
    fn no_common_ancestors() {
      let path1 = "one/two/three/file.md";
      let path2 = "alpha/beta/file.md";
      let have = super::super::relative(path1, path2);
      let want = "../../../alpha/beta/file.md";
      assert_eq!(have, want);
    }

    #[test]
    fn same_dir() {
      let path1 = "one/two/three/file.md";
      let path2 = "one/two/three/other.md";
      let have = super::super::relative(path1, path2);
      let want = "other.md";
      assert_eq!(have, want);
    }

    #[test]
    fn root_dir() {
      let path1 = "file.md";
      let path2 = "other.md";
      let have = super::super::relative(path1, path2);
      let want = "other.md";
      assert_eq!(have, want);
    }

    #[test]
    fn from_subdir() {
      let path1 = "sub/file.md";
      let path2 = "other.md";
      let have = super::super::relative(path1, path2);
      let want = "../other.md";
      assert_eq!(have, want);
    }

    #[test]
    fn into_subdir() {
      let path1 = "file.md";
      let path2 = "sub/other.md";
      let have = super::super::relative(path1, path2);
      let want = "sub/other.md";
      assert_eq!(have, want);
    }
  }
}
