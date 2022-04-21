use crate::{Issue, Location};
use regex::Regex;
use schemars::JsonSchema;
use serde::Deserialize;
use std::fs::File;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

/// Tikibase configuration data
#[derive(Clone, Deserialize, Debug, Default, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    /// enables bi-directional links
    pub bidi_links: Option<bool>,

    /// Names of filesystem entries to ignore in this directory.
    pub ignore: Option<Vec<String>>,

    /// the allowed section titles
    pub sections: Option<Vec<String>>,

    /// regex with a single capture group to extract a shorter title for links to notes
    pub title_reg_ex: Option<String>,

    /// link to the JSON-Schema definition for this file
    #[serde(rename(deserialize = "$schema"))]
    pub schema: Option<String>,
}

impl Config {
    /// indicates whether the given file should be ignored
    pub fn ignore(&self, file_path: &str) -> bool {
        match &self.ignore {
            Some(ignores) => ignores.iter().any(|ignore| ignore == file_path),
            None => false,
        }
    }

    /// provides the regular expression as a proper Regex instance
    pub fn title_regex(&self) -> Result<Option<Regex>, Issue> {
        match &self.title_reg_ex {
            Some(text) => match Regex::new(text) {
                Ok(regex) => Ok(Some(regex)),
                Err(err) => Err(Issue::InvalidTitleRegex {
                    regex: text.into(),
                    problem: err.to_string(),
                    file: PathBuf::from("tikibase.json"),
                }),
            },
            None => Ok(None),
        }
    }
}

/// reads the config file
pub fn load<P: AsRef<Path>>(dir: P) -> LoadResult {
    let config_path = dir.as_ref().join("tikibase.json");
    let file = match File::open(&config_path) {
        Ok(reader) => reader,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => return LoadResult::NotFound,
            _ => {
                return LoadResult::Error(Issue::CannotReadConfigurationFile {
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
    match serde_json::from_reader(file) {
        Ok(config) => LoadResult::Loaded(config),
        Err(e) => LoadResult::Error(Issue::InvalidConfigurationFile {
            message: e.to_string(),
            location: Location {
                file: PathBuf::from("tikibase.json"),
                line: e.line() as u32,
                start: e.column() as u32,
                end: e.column() as u32,
            },
        }),
    }
}

#[derive(Debug, PartialEq)]
pub enum LoadResult {
    Loaded(Config),
    NotFound,
    Error(Issue),
}

#[cfg(test)]
mod tests {

    mod ignore {
        use crate::Config;

        #[test]
        fn direct_match() {
            let config = Config {
                ignore: Some(vec!["Makefile".into()]),
                ..Config::default()
            };
            assert!(config.ignore("Makefile"));
        }

        // #[test]
        // fn glob_match() {
        //     let config = Config {
        //         ignore: Some(vec!["!**/Makefile".into()]),
        //         ..Config::default()
        //     };
        //     let have = config.ignore(PathBuf::from("foo/bar/Makefile"));
        //     assert!(!have);
        // }

        #[test]
        fn no_match() {
            let config = Config {
                ignore: Some(vec!["Makefile".into()]),
                ..Config::default()
            };
            assert!(!config.ignore("other"));
        }

        #[test]
        fn no_ignores() {
            let config = Config {
                ignore: None,
                ..Config::default()
            };
            assert!(!config.ignore("file"));
        }
    }

    mod load {
        use super::super::{load, Config};
        use crate::config::LoadResult;
        use crate::{test, Issue, Location};
        use std::path::PathBuf;

        #[test]
        fn no_config_file() {
            let have = load(test::tmp_dir());
            let want = LoadResult::NotFound;
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn empty_config_file() {
            let dir = test::tmp_dir();
            test::create_file("tikibase.json", "{}", &dir);
            let have = load(&dir);
            let want = LoadResult::Loaded(Config {
                bidi_links: None,
                sections: None,
                ignore: None,
                schema: None,
                title_reg_ex: None,
            });
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn valid_config_file() {
            let dir = test::tmp_dir();
            let give = r#"
            {
              "bidiLinks": true,
              "sections": [ "one", "two" ],
              "ignore": [ "foo" ]
            }
            "#;
            test::create_file("tikibase.json", give, &dir);
            let have = load(&dir);
            let want = LoadResult::Loaded(Config {
                bidi_links: Some(true),
                sections: Some(vec!["one".into(), "two".into()]),
                ignore: Some(vec!["foo".into()]),
                schema: None,
                title_reg_ex: None,
            });
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn unknown_field() {
            let dir = test::tmp_dir();
            let give = r#"
            {
              "foo": true,
            }
            "#;
            test::create_file("tikibase.json", give, &dir);
            let have = load(&dir);
            let want = LoadResult::Error(Issue::InvalidConfigurationFile {
                message: "unknown field `foo`, expected one of `bidiLinks`, `ignore`, `sections`, `titleRegEx`, `$schema` at line 3 column 20".into(),
                location: Location {
                    file: PathBuf::from("tikibase.json"),
                    line: 3,
                    start: 20,
                    end: 20,
                },
            });
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
            let want = LoadResult::Error(Issue::InvalidConfigurationFile {
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
