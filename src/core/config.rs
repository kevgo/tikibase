use serde::Deserialize;
use std::fs::File;
use std::io::ErrorKind;
use std::path::Path;

/// Tikibase configuration data
#[derive(Deserialize, Default, PartialEq, Debug)]
pub struct Data {
    /// the allowed section types
    pub section_names: Option<Vec<String>>,

    /// files to ignore
    pub ignore: Option<Vec<String>>,
}

/// reads the config file
pub fn load<P: AsRef<Path>>(dir: P) -> Result<Data, String> {
    let config_path = dir.as_ref().join("tikibase.json");
    let file = match File::open(config_path) {
        Ok(reader) => reader,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => return Ok(Default::default()),
            _ => {
                return Err(format!(
                    "Cannot access configuration file (tikibase.json): {}",
                    e
                ))
            }
        },
    };
    serde_json::from_reader(file)
        .map_err(|e| format!("Configuration file has invalid structure: {}", e))
}

#[cfg(test)]
mod tests {
    use super::Data;

    #[test]
    fn defaults() {
        let default_config: Data = Default::default();
        assert_eq!(default_config.section_names, None)
    }

    mod load {
        use crate::testhelpers::{create_file, tmp_dir};

        #[test]
        fn no_config_file() {
            let dir = tmp_dir();
            let have = super::super::load(dir).unwrap();
            let want = super::super::Data {
                section_names: None,
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
                section_names: None,
                ignore: None,
            };
            assert_eq!(have, want);
        }

        #[test]
        fn valid_config_file() {
            let dir = tmp_dir();
            let content = r#"
            {
              "section_names": [ "one", "two" ]
            }
            "#;
            create_file("tikibase.json", content, &dir);
            let have = super::super::load(&dir).unwrap();
            let want = super::super::Data {
                section_names: Some(vec!["one".to_string(), "two".to_string()]),
                ignore: None,
            };
            assert_eq!(have, want);
        }

        #[test]
        fn invalid_config_file() {
            let dir = tmp_dir();
            let content = r#"{
    "section_names": [
}
"#;
            create_file("tikibase.json", content, &dir);
            match super::super::load(&dir) {
                Err(e) => assert_eq!(
                    e,
                    "Configuration file has invalid structure: expected value at line 3 column 1"
                ),
                Ok(_) => panic!(),
            }
        }
    }
}
