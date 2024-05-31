/// trims whitespace from the end of this string,
/// inline without allocating a new String
pub fn trim_end(text: &mut String) {
  while text.ends_with(char::is_whitespace) {
    text.truncate(text.len() - 1);
  }
}

#[cfg(test)]
mod tests {

  mod trim_end {
    use super::super::trim_end;
    use big_s::S;

    #[test]
    fn whitespaces() {
      let mut s = S("Foo\n\n");
      trim_end(&mut s);
      assert_eq!(s, "Foo");
    }

    #[test]
    fn no_whitespace() {
      let mut s = S("Foo");
      trim_end(&mut s);
      assert_eq!(s, "Foo");
    }
  }
}
