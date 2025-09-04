use crate::check::{Issue, Location};
use crate::database::Section;
use big_s::S;
use camino::Utf8Path;
use fs_err::File;
use merge::Merge;
use regex::Regex;
use schemars::JsonSchema;
use serde::Deserialize;
use std::io::ErrorKind;

/// Tikibase configuration data
#[derive(Clone, Deserialize, Debug, Default, Eq, JsonSchema, Merge, PartialEq)]
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
  #[serde(rename = "$schema")]
  pub schema: Option<String>,

  /// whether documents without links are allowed
  pub standalone_docs: Option<bool>,
}

impl Config {
  /// indicates whether the given file should be ignored
  #[must_use]
  pub fn ignore(&self, file_path: &str) -> bool {
    match &self.ignore {
      Some(ignores) => ignores.iter().any(|ignore| ignore == file_path),
      None => false,
    }
  }

  /// indicates whether the given title matches one of the allowed titles
  #[must_use]
  pub fn matching_title(&self, title: &str) -> bool {
    match &self.sections {
      // HACK: see https://github.com/rust-lang/rust/issues/42671
      Some(sections) => sections
        .iter()
        .any(|config_section| config_section == title),
      None => true,
    }
  }

  /// provides the configured section title corresponding to the given human title
  #[must_use]
  pub fn section_with_human_title(&self, human_title: &str) -> Option<&str> {
    if let Some(sections) = &self.sections {
      for section_title in sections {
        let (_level, start) = Section::parse_title(section_title);
        let section_human_title = &section_title[start..];
        if section_human_title == human_title {
          return Some(section_title);
        }
      }
    }
    None
  }

  /// indicates whether Tikibase should check for standalone documents
  #[must_use]
  pub fn check_standalone_docs(&self) -> bool {
    match self.standalone_docs {
      Some(flag) => !flag,
      None => true,
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
          file: S("tikibase.json"),
        }),
      },
      None => Ok(None),
    }
  }
}

/// reads the config file
pub fn load<P: AsRef<Utf8Path>>(dir: P) -> LoadResult {
  let config_path = dir.as_ref().join("tikibase.json");
  let file = match File::open(config_path) {
    Ok(reader) => reader,
    Err(e) => match e.kind() {
      ErrorKind::NotFound => return LoadResult::NotFound,
      _ => {
        return LoadResult::Error(Issue::CannotReadConfigurationFile {
          message: e.to_string(),
          location: Location {
            file: S("tikibase.json"),
            line: 0,
            start: 0,
            end: 0,
          },
        });
      }
    },
  };
  match serde_json::from_reader(file) {
    Ok(config) => LoadResult::Loaded(config),
    Err(e) => LoadResult::Error(Issue::InvalidConfigurationFile {
      message: e.to_string(),
      location: Location {
        file: S("tikibase.json"),
        line: e.line() as u32,
        start: e.column() as u32,
        end: e.column() as u32,
      },
    }),
  }
}

#[derive(Debug, Eq, PartialEq)]
pub enum LoadResult {
  Loaded(Config),
  NotFound,
  Error(Issue),
}

#[cfg(test)]
mod tests {

  mod check_standalone_docs {
    use crate::Config;

    #[test]
    fn none() {
      let config = Config {
        standalone_docs: None,
        ..Config::default()
      };
      assert!(config.check_standalone_docs());
    }

    #[test]
    fn enabled() {
      let config = Config {
        standalone_docs: Some(true),
        ..Config::default()
      };
      assert!(!config.check_standalone_docs());
    }

    #[test]
    fn disabled() {
      let config = Config {
        standalone_docs: Some(false),
        ..Config::default()
      };
      assert!(config.check_standalone_docs());
    }
  }

  mod ignore {
    use crate::Config;
    use big_s::S;

    #[test]
    fn direct_match() {
      let config = Config {
        ignore: Some(vec![S("Makefile")]),
        ..Config::default()
      };
      assert!(config.ignore("Makefile"));
    }

    #[test]
    fn no_match() {
      let config = Config {
        ignore: Some(vec![S("Makefile")]),
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
    use super::super::{Config, load};
    use crate::check::{Issue, Location};
    use crate::config::LoadResult;
    use crate::test;
    use big_s::S;

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
        standalone_docs: None,
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
        sections: Some(vec![S("one"), S("two")]),
        ignore: Some(vec![S("foo")]),
        schema: None,
        title_reg_ex: None,
        standalone_docs: None,
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
        message: S(
          "unknown field `foo`, expected one of `bidiLinks`, `ignore`, `sections`, `titleRegEx`, `$schema`, `standaloneDocs` at line 3 column 20",
        ),
        location: Location {
          file: S("tikibase.json"),
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
        message: S("expected value at line 3 column 1"),
        location: Location {
          file: S("tikibase.json"),
          line: 3,
          start: 1,
          end: 1,
        },
      });
      pretty::assert_eq!(have, want);
    }
  }

  mod matching_title {
    use crate::Config;
    use big_s::S;

    #[test]
    fn matches() {
      let config = Config {
        sections: Some(vec![S("one"), S("two")]),
        ..Config::default()
      };
      assert!(config.matching_title("two"));
    }

    #[test]
    fn no_match() {
      let config = Config {
        sections: Some(vec![S("one"), S("two")]),
        ..Config::default()
      };
      assert!(!config.matching_title("three"));
    }

    #[test]
    fn not_defined() {
      let config = Config {
        sections: None,
        ..Config::default()
      };
      assert!(config.matching_title("anything"));
    }
  }

  mod merge {
    use crate::Config;
    use big_s::S;
    use merge::Merge;

    #[test]
    fn merge_default() {
      let mut config1 = Config {
        bidi_links: Some(true),
        ignore: Some(vec![S("one"), S("two")]),
        sections: Some(vec![S("hello"), S("bye")]),
        title_reg_ex: Some(S("config2regex")),
        schema: Some(S("config2schema")),
        standalone_docs: Some(true),
      };
      let config2 = Config::default();
      let old_config_1 = config1.clone();
      config1.merge(config2);
      assert_eq!(config1, old_config_1);
    }

    #[test]
    fn merge_into_default() {
      let mut config1 = Config::default();
      let config2 = Config {
        bidi_links: Some(true),
        ignore: Some(vec![S("one"), S("two")]),
        sections: Some(vec![S("hello"), S("bye")]),
        title_reg_ex: Some(S("config2regex")),
        schema: Some(S("config2schema")),
        standalone_docs: Some(true),
      };
      config1.merge(config2.clone());
      assert_eq!(config1, config2);
    }

    #[test]
    fn both_have_values() {
      let mut config1 = Config {
        bidi_links: Some(true),
        ignore: Some(vec![S("one"), S("two")]),
        sections: Some(vec![S("hello"), S("bye")]),
        title_reg_ex: Some(S("config2regex")),
        schema: Some(S("config2schema")),
        standalone_docs: Some(true),
      };
      let config2 = Config {
        bidi_links: Some(true),
        ignore: Some(vec![S("one"), S("two")]),
        sections: Some(vec![S("hello"), S("bye")]),
        title_reg_ex: Some(S("config2regex")),
        schema: Some(S("config2schema")),
        standalone_docs: Some(true),
      };
      config1.merge(config2.clone());
      assert_eq!(config1, config2);
    }
  }
}
