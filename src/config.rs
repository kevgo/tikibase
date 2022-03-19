use crate::issues::Issue;
use serde::Deserialize;
use std::fs::File;
use std::io::ErrorKind;
use std::path::Path;

/// Tikibase configuration data
#[derive(Deserialize, Default, PartialEq, Debug)]
pub struct Data {
    /// the allowed section types
    pub sections: Option<Vec<String>>,

    /// files to ignore
    pub ignore: Option<Vec<String>>,
}

/// reads the config file
pub fn load<P: AsRef<Path>>(dir: P) -> Result<Data, Issue> {
    let config_path = dir.as_ref().join("tikibase.json");
    let file = match File::open(config_path) {
        Ok(reader) => reader,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => return Ok(Data::default()),
            _ => {
                return Err(Issue::CannotReadConfigurationFile {
                    message: e.to_string(),
                })
            }
        },
    };
    serde_json::from_reader(file).map_err(|e: serde_json::Error| Issue::InvalidConfigurationFile {
        message: e.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::Data;

    #[test]
    fn defaults() {
        let default_config = Data::default();
        assert_eq!(default_config.sections, None);
    }

    mod load {
        use crate::testhelpers::{create_file, tmp_dir};
        use crate::Issue;

        #[test]
        fn no_config_file() {
            let dir = tmp_dir();
            let have = super::super::load(dir).unwrap();
            let want = super::super::Data {
                sections: None,
                ignore: None,
            };
            assert_eq!(have, want);
        }

        #[test]
        fn empty_config_file() {
            let dir = tmp_dir();
            create_file("tikibase.json", "{}", &dir);
            let have = super::super::load(&dir).unwrap();
            let want = super::super::Data {
                sections: None,
                ignore: None,
            };
            assert_eq!(have, want);
        }

        #[test]
        fn valid_config_file() {
            let dir = tmp_dir();
            let content = r#"
            {
              "sections": [ "one", "two" ]
            }
            "#;
            create_file("tikibase.json", content, &dir);
            let have = super::super::load(&dir).unwrap();
            let want = super::super::Data {
                sections: Some(vec!["one".into(), "two".into()]),
                ignore: None,
            };
            assert_eq!(have, want);
        }

        #[test]
        fn invalid_config_file() {
            let dir = tmp_dir();
            let content = r#"{
    "sections": [
}
"#;
            create_file("tikibase.json", content, &dir);
            let have = super::super::load(&dir);
            let want = Err(Issue::InvalidConfigurationFile {
                message: "expected value at line 3 column 1".into(),
            });
            assert_eq!(have, want)
        }
    }
}
