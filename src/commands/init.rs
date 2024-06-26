use super::Outcome;
use crate::check::Issue;
use crate::database::paths;
use big_s::S;
use fs_err as fs;
use indoc::indoc;

#[must_use]
pub fn init(dir: &str) -> Outcome {
  match fs::write(paths::join(dir, "tikibase.json"), template()) {
    Ok(()) => Outcome::default(),
    Err(err) => Outcome::from_issue(Issue::CannotWriteConfigFile {
      file: S("tikibase.json"),
      message: err.to_string(),
    }),
  }
}

/// provides the content of the initial config file
fn template() -> &'static str {
  indoc! {"
    {
      \"$schema\": \"https://raw.githubusercontent.com/kevgo/tikibase/main/doc/tikibase.schema.json\"
    }
    "}
}

#[cfg(test)]
mod tests {
  use super::template;
  use crate::Config;

  #[test]
  fn init() {
    // verify that the template can be parsed into a valid Config struct without errors
    let _have: Config = serde_json::from_str(template()).unwrap();
  }
}
