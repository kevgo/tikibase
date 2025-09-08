/// resolves elements like "../" and "./" in the given string
pub fn normalize(path: &str) -> String {
  let mut segments: Vec<&str> = vec![];
  let mut parents: u16 = 0;
  let mut segment_start: usize = 0;
  for (i, current_char) in path.chars().enumerate() {
    if current_char == '/' {
      match segment(path, segment_start, i) {
        "." => {}
        ".." => parents += 1,
        segment => {
          pop_segments(&mut segments, &mut parents);
          segments.push(segment);
        }
      }
      segment_start = i;
    }
  }
  match segment(path, segment_start, path.len()) {
    "." => {}
    ".." => parents += 1,
    segment => {
      pop_segments(&mut segments, &mut parents);
      segments.push(segment);
    }
  }
  pop_segments(&mut segments, &mut parents);
  segments.join("/")
}

/// part of normalize
fn pop_segments(segments: &mut Vec<&str>, parents: &mut u16) {
  while *parents > 0 && segments.len() > 0 {
    segments.pop();
    *parents -= 1;
  }
}

/// part of `normalize`
fn segment(path: &str, start: usize, end: usize) -> &str {
  if start > 0 {
    &path[start + 1..end]
  } else {
    &path[..end]
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
    fn go_above_root() {
      let give = "one/../../1.md";
      let have = super::super::normalize(give);
      let want = S("../1.md");
      assert_eq!(have, want);
    }
  }
}
