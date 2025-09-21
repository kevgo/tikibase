#[derive(Debug, Eq, PartialEq)]
pub struct Result<'a> {
  file: String,
  line: &'a str,
  start: u32,
  end: u32,
}
