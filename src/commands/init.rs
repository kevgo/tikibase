use crate::{Issue, Outcome};
use indoc::indoc;
use std::fs;
use std::path::{Path, PathBuf};

pub fn init(dir: &Path) -> Outcome {
    match fs::write(dir.join("tikibase.json"), template()) {
        Ok(_) => Outcome::default(),
        Err(err) => Outcome::from_issue(Issue::CannotWriteConfigFile {
            file: PathBuf::from("tikibase.json"),
            message: err.to_string(),
        }),
    }
}

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
        let _have: Config = serde_json::from_str(template()).unwrap();
    }
}
