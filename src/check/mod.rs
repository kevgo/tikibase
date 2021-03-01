use crate::core::tikibase::Tikibase;
mod empty_sections;
mod section_capitalization;

pub fn run() {
  let base = Tikibase::in_dir(".");
  for error in section_capitalization::find(&base) {
    println!(
      "- mixed capitalization of sections: \"{}\"",
      error.variants.join("\", \"")
    );
  }
  for section in empty_sections::find(&base) {
    println!(
      "- {}:{} empty section",
      section.path.to_str().unwrap(),
      section.line_number
    );
  }
}
