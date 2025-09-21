pub fn join(path1: &str, path2: &str) -> String {
  if path1.is_empty() || path2.is_empty() {
    format!("{path1}{path2}")
  } else {
    format!("{path1}/{path2}")
  }
}

#[cfg(test)]
mod tests {

  mod join {
    use big_s::S;

    #[test]
    fn two_paths() {
      let have = super::super::join("one", "two");
      let want = S("one/two");
      assert_eq!(have, want);
    }

    #[test]
    fn first_path_empty() {
      let have = super::super::join("", "two");
      let want = S("two");
      assert_eq!(have, want);
    }

    #[test]
    fn second_path_empty() {
      let have = super::super::join("one", "");
      let want = S("one");
      assert_eq!(have, want);
    }
  }
}
