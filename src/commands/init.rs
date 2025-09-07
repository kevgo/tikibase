use crate::database::paths;
use crate::prelude::*;
use big_s::S;
use fs_err as fs;
use indoc::indoc;

pub fn init(dir: &str) -> Result<()> {
  let path = paths::join(dir, "tikibase.json");
  fs::write(path, template()).map_err(|err| UserError::CannotWriteFile {
    filename: S("tikibase.json"),
    reason: err.to_string(),
  })
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
