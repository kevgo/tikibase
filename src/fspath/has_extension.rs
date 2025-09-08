/// case-insensitive comparison of file extensions
pub fn has_extension(path: &str, given_ext: &str) -> bool {
  let path_ext = path.rsplit('.').next().unwrap();
  path_ext.eq_ignore_ascii_case(given_ext)
}
