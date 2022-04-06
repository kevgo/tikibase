use crate::{Issue, Location};
use schemars::JsonSchema;
use serde::Deserialize;
use std::fs::File;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

/// Tikibase configuration data
#[derive(Deserialize, Debug, Default, JsonSchema, PartialEq)]
pub struct Config {
    /// the allowed section titles
    pub sections: Option<Vec<String>>,

    /// glob overrides
    pub globs: Option<Vec<String>>,
}

impl Config {
    /// indicates whether the given file should be ignored
    pub fn ignore<P: AsRef<Path>>(&self, file_path: P) -> bool {
        match &self.globs {
            Some(ignores) => {
                let file_path = file_path.as_ref().as_os_str().to_string_lossy();
                ignores.iter().any(|ignore| ignore == &file_path)
            }
            None => false,
        }
    }
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
                        start: 0,
                        end: 0,
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
            start: e.column() as u32,
            end: e.column() as u32,
        },
    })
}

#[cfg(test)]
mod tests {

    mod ignore {
        use crate::Config;
        use std::path::PathBuf;

        #[test]
        fn direct_match() {
            let config = Config {
                globs: Some(vec!["!Makefile".into()]),
                ..Config::default()
            };
            let have = config.ignore(PathBuf::from("Makefile"));
            assert!(!have);
        }

        #[test]
        fn glob_match() {
            let config = Config {
                globs: Some(vec!["!**/Makefile".into()]),
                ..Config::default()
            };
            let have = config.ignore(PathBuf::from("foo/bar/Makefile"));
            assert!(!have);
        }

        #[test]
        fn no_match() {
            let config = Config {
                globs: Some(vec!["Makefile".into()]),
                ..Config::default()
            };
            let have = config.ignore(PathBuf::from("other"));
            assert!(!have);
        }

        #[test]
        fn no_ignores() {
            let config = Config {
                globs: None,
                ..Config::default()
            };
            let have = config.ignore(PathBuf::from("file"));
            assert!(!have);
        }
    }

    mod load {
        use super::super::{load, Config};
        use crate::{test, Issue, Location};
        use std::path::PathBuf;

        #[test]
        fn no_config_file() {
            let have = load(test::tmp_dir()).unwrap();
            let want = Config {
                sections: None,
                globs: None,
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
                globs: None,
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
                globs: None,
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
                    start: 1,
                    end: 1,
                },
            });
            pretty::assert_eq!(have, want);
        }
    }
}
