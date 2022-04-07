use crate::{Issue, Location, Outcome};
use indoc::indoc;
use std::fs;
use std::path::PathBuf;

pub fn init() -> Outcome {
    match fs::write("tikibase.json", template()) {
        Ok(_) => Outcome::default(),
        Err(err) => Outcome::from_issue(Issue::CannotWriteConfigFile {
            message: err.to_string(),
            location: Location {
                file: PathBuf::from("tikibase.json"),
                line: 0,
                start: 0,
                end: 0,
            },
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
        let _: Config = serde_json::from_str(template()).unwrap();
    }
}
