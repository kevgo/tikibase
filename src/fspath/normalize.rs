/// resolves elements like "../" and "./" in the given string
pub fn normalize(path: &str) -> String {
  let mut segments: Vec<&str> = path
    .split("/")
    .filter(|segment| *segment != "" && *segment != ".")
    .collect();

  let mut changed: bool;
  loop {
    (segments, changed) = simplify(segments);
    if !changed {
      break;
    }
  }

  segments.join("/")
}

fn simplify(segments: Vec<&str>) -> (Vec<&str>, bool) {
  let mut result = vec![];
  let mut last = None;
  let mut changed = false;
  for segment in segments {
    if segment == ".." && last.is_some() {
      last = None;
      changed = true;
      continue;
    }
    if let Some(last_seg) = last {
      result.push(last_seg);
      last = Some(segment);
      continue;
    }
    last = Some(segment);
  }
  if let Some(last_seg) = last {
    result.push(last_seg);
  }
  (result, changed)
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
