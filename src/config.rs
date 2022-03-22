use crate::{Issue, Location};
use serde::Deserialize;
use std::fs::File;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

/// Tikibase configuration data
#[derive(Deserialize, Default, PartialEq, Debug)]
pub struct Config {
    /// the allowed section types
    pub sections: Option<Vec<String>>,

    /// files to ignore
    pub ignore: Option<Vec<String>>,
}

/// reads the config file
pub fn load<P: AsRef<Path>>(dir: P) -> Result<Config, Issue> {
    let config_path = dir.as_ref().join("tikibase.json");
    let file = match File::open(&config_path) {
        Ok(reader) => reader,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => return Ok(Config::default()),
            _ => {
                return Err(Issue::CannotReadConfigurationFile {
                    message: e.to_string(),
                    location: Location {
                        file: PathBuf::from("tikibase.json"),
                        line: 0,
                    },
                })
            }
        },
    };
    serde_json::from_reader(file).map_err(|e: serde_json::Error| Issue::InvalidConfigurationFile {
        message: e.to_string(),
        location: Location {
            file: PathBuf::from("tikibase.json"),
            line: e.line() as u32,
        },
    })
}

#[cfg(test)]
mod tests {

    mod load {
        use super::super::{load, Config};
        use crate::{test, Issue, Location};
        use std::path::PathBuf;

        #[test]
        fn no_config_file() {
            let have = load(test::tmp_dir()).unwrap();
            let want = Config {
                sections: None,
                ignore: None,
            };
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn empty_config_file() {
            let dir = test::tmp_dir();
            test::create_file("tikibase.json", "{}", &dir);
            let have = load(&dir).unwrap();
            let want = Config {
                sections: None,
                ignore: None,
            };
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn valid_config_file() {
            let dir = test::tmp_dir();
            let give = r#"
            {
              "sections": [ "one", "two" ]
            }
            "#;
            test::create_file("tikibase.json", give, &dir);
            let have = load(&dir).unwrap();
            let want = Config {
                sections: Some(vec!["one".into(), "two".into()]),
                ignore: None,
            };
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn invalid_config_file() {
            let dir = test::tmp_dir();
            let give = r#"{
    "sections": [
}
"#;
            test::create_file("tikibase.json", give, &dir);
            let have = load(&dir);
            let want = Err(Issue::InvalidConfigurationFile {
                message: "expected value at line 3 column 1".into(),
                location: Location {
                    file: PathBuf::from("tikibase.json"),
                    line: 3,
                },
            });
            pretty::assert_eq!(have, want)
        }
    }
}
