use crate::core::tikibase::Tikibase;
mod section_capitalization;

#[derive(PartialEq, Debug)]
pub enum Error {
  MixedCapSection { variants: Vec<String> },
}

pub fn run() {
  let tb = Tikibase::in_dir(".");
  let mut errors = Vec::new();
  errors.append(&mut section_capitalization::find(&tb));
  for error in errors {
    match error {
      Error::MixedCapSection { variants } => {
        println!(
          "- mixed capitalization of sections: \"{}\"",
          variants.join("\", \"")
        );
      }
    }
  }
}
