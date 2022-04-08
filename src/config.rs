use crate::{Issue, Location};
use regex::Regex;
use schemars::JsonSchema;
use serde::Deserialize;
use std::fs::File;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::str::FromStr;

/// Tikibase configuration data
#[derive(Deserialize, Debug, Default, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    /// enables bi-directional links
    pub bidi_links: Option<bool>,

    /// Glob overrides. Tikibase looks at all files that aren't git-ignored.
    /// With this setting you can fine-tune the files Tikibase looks at
    /// using glob expressions. To exclude files, precede the glob with a `!`.
    pub globs: Option<Vec<String>>,

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
    pub fn ignore<P: AsRef<Path>>(&self, file_path: P) -> bool {
        match &self.globs {
            Some(ignores) => {
                let file_path = file_path.as_ref().as_os_str().to_string_lossy();
                ignores.iter().any(|ignore| ignore == &file_path)
            }
            None => false,
        }
    }
    pub fn title_regex(&self) -> Result<Option<Regex>, Issue> {
        match &self.title_reg_ex {
            Some(text) => match Regex::from_str(&text) {
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
                bidi_links: None,
                sections: None,
                globs: None,
                schema: None,
                title_reg_ex: None,
            };
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn empty_config_file() {
            let dir = test::tmp_dir();
            test::create_file("tikibase.json", "{}", &dir);
            let have = load(&dir).unwrap();
            let want = Config {
                bidi_links: None,
                sections: None,
                globs: None,
                schema: None,
                title_reg_ex: None,
            };
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn valid_config_file() {
            let dir = test::tmp_dir();
            let give = r#"
            {
              "bidiLinks": true,
              "sections": [ "one", "two" ],
              "globs": [ "**/foo" ]
            }
            "#;
            test::create_file("tikibase.json", give, &dir);
            let have = load(&dir).unwrap();
            let want = Config {
                bidi_links: Some(true),
                sections: Some(vec!["one".into(), "two".into()]),
                globs: Some(vec!["**/foo".into()]),
                schema: None,
                title_reg_ex: None,
            };
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
            let want = Err(Issue::InvalidConfigurationFile {
                message: "unknown field `foo`, expected one of `bidiLinks`, `globs`, `sections`, `titleRegEx`, `$schema` at line 3 column 20".into(),
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
