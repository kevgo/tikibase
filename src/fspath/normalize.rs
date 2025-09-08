/// resolves elements like "../" and "./" in the given string
pub fn normalize(path: &str) -> String {
  let segments = path.split(std::path::MAIN_SEPARATOR_STR);
  let mut result: Vec<&str> = vec![];
  let mut parents: u16 = 0;
  for segment in segments {
    match segment {
      "." => {}
      ".." => {
        parents += 1;
      }
      segment => {
        pop_segments(&mut result, &mut parents);
        result.push(segment);
      }
    }
  }
  pop_segments(&mut result, &mut parents);
  result.join(std::path::MAIN_SEPARATOR_STR)
}

/// part of normalize
fn pop_segments(segments: &mut Vec<&str>, parents: &mut u16) {
  while *parents > 0 {
    if segments.is_empty() {
      *parents -= 1;
      segments.push("..");
      return;
    }
    segments.pop();
    *parents -= 1;
  }
}

#[cfg(test)]
mod tests {

  mod normalize {
    use big_s::S;

    #[test]
    fn parent_placeholders() {
      let give = "one/three/../two/three/../../new.md";
      let want = S("one/new.md");
      let have = super::super::normalize(give);
      assert_eq!(have, want);
    }

    #[test]
    fn trailing_parent_placeholder() {
      let give = "one/two/three/../..";
      let want = S("one");
      let have = super::super::normalize(give);
      assert_eq!(have, want);
    }

    #[test]
    fn current_placeholders() {
      let give = "./one/./././two/./three.md";
      let want = S("one/two/three.md");
      let have = super::super::normalize(give);
      assert_eq!(have, want);
    }

    #[test]
    fn single_segment() {
      let give = "2.md";
      let want = S("2.md");
      let have = super::super::normalize(give);
      assert_eq!(have, want);
    }

    #[test]
    fn no_placeholders() {
      let give = "one/two/2.md";
      let want = S("one/two/2.md");
      let have = super::super::normalize(give);
      assert_eq!(have, want);
    }

    #[test]
    fn go_below_root() {
      let give = "../1.md";
      let have = super::super::normalize(give);
      let want = S("../1.md");
      assert_eq!(have, want);
    }

    #[test]
    fn go_up_and_then_down_below_root() {
      let give = "one/../../1.md";
      let have = super::super::normalize(give);
      let want = S("../1.md");
      assert_eq!(have, want);
    }
  }
}
