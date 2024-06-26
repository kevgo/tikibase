/// footnote definitions and references contained in a document
#[derive(Debug, Default, Eq, PartialEq)]
pub struct Footnotes {
  pub definitions: Vec<Footnote>,
  pub references: Vec<Footnote>,
}

impl Footnotes {
  fn contains_definition(&self, identifier: &str) -> bool {
    self
      .definitions
      .iter()
      .any(|definition| definition.identifier == identifier)
  }

  /// indicates whether this footnotes collection contains a footnote reference with the given identifier
  fn contains_reference(&self, identifier: &str) -> bool {
    self
      .references
      .iter()
      .any(|reference| reference.identifier == identifier)
  }

  /// provides footnote definitions that aren't referenced in the text
  pub fn missing_references(&self) -> impl Iterator<Item = &Footnote> {
    self
      .references
      .iter()
      .filter(|reference| !self.contains_definition(&reference.identifier))
  }

  /// provides footnote references that have no definition
  pub fn unused_definitions(&self) -> impl Iterator<Item = &Footnote> {
    self
      .definitions
      .iter()
      .filter(|definition| !self.contains_reference(&definition.identifier))
  }
}

/// reference to a footnote
#[derive(Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Footnote {
  /// the textual identifier of the footnote
  pub identifier: String,
  /// the line on which this footnote exists
  pub line: u32,
  /// where on the line the footnote starts
  pub start: u32,
  /// where on the line the footnote ends
  pub end: u32,
}

#[cfg(test)]
mod tests {

  mod contains_definition {
    use crate::database::{Footnote, Footnotes};
    use big_s::S;

    #[test]
    fn contains() {
      let give = Footnotes {
        definitions: vec![Footnote {
          identifier: S("f1"),
          ..Footnote::default()
        }],
        references: vec![],
      };
      assert!(give.contains_definition("f1"));
    }

    #[test]
    fn does_not_contain() {
      let give = Footnotes {
        definitions: vec![Footnote {
          identifier: S("f1"),
          ..Footnote::default()
        }],
        references: vec![],
      };
      assert!(!give.contains_definition("f2"));
    }
  }

  mod contains_reference {
    use crate::database::{Footnote, Footnotes};
    use big_s::S;

    #[test]
    fn contains() {
      let give = Footnotes {
        definitions: vec![],
        references: vec![Footnote {
          identifier: S("f1"),
          ..Footnote::default()
        }],
      };
      assert!(give.contains_reference("f1"));
    }

    #[test]
    fn does_not_contain() {
      let give = Footnotes {
        definitions: vec![],
        references: vec![Footnote {
          identifier: S("f1"),
          ..Footnote::default()
        }],
      };
      assert!(!give.contains_reference("f2"));
    }
  }

  mod missing_references {
    use crate::database::{Footnote, Footnotes};
    use big_s::S;

    #[test]
    fn missing() {
      let give = Footnotes {
        definitions: vec![Footnote {
          identifier: S("f2"),
          ..Footnote::default()
        }],
        references: vec![
          Footnote {
            identifier: S("f1"),
            ..Footnote::default()
          },
          Footnote {
            identifier: S("f2"),
            ..Footnote::default()
          },
        ],
      };
      let have = give.missing_references().map(|f| f.identifier.as_str());
      itertools::assert_equal(have, vec!["f1"]);
    }

    #[test]
    fn all_used() {
      let give = Footnotes {
        definitions: vec![
          Footnote {
            identifier: S("f1"),
            ..Footnote::default()
          },
          Footnote {
            identifier: S("f2"),
            ..Footnote::default()
          },
        ],
        references: vec![
          Footnote {
            identifier: S("f1"),
            ..Footnote::default()
          },
          Footnote {
            identifier: S("f2"),
            ..Footnote::default()
          },
        ],
      };
      let have = give.missing_references();
      itertools::assert_equal(have, Vec::<&Footnote>::new());
    }
  }

  mod unused_definitions {
    use crate::database::{Footnote, Footnotes};
    use big_s::S;

    #[test]
    fn missing() {
      let give = Footnotes {
        definitions: vec![
          Footnote {
            identifier: S("f1"),
            ..Footnote::default()
          },
          Footnote {
            identifier: S("f2"),
            ..Footnote::default()
          },
        ],
        references: vec![Footnote {
          identifier: S("f2"),
          ..Footnote::default()
        }],
      };
      let have = give.unused_definitions().map(|d| &d.identifier);
      let want = vec!["f1"];
      itertools::assert_equal(have, want);
    }

    #[test]
    fn all_used() {
      let give = Footnotes {
        definitions: vec![
          Footnote {
            identifier: S("f1"),
            ..Footnote::default()
          },
          Footnote {
            identifier: S("f2"),
            ..Footnote::default()
          },
        ],
        references: vec![
          Footnote {
            identifier: S("f1"),
            ..Footnote::default()
          },
          Footnote {
            identifier: S("f2"),
            ..Footnote::default()
          },
        ],
      };
      let have = give.unused_definitions();
      itertools::assert_equal(have, Vec::<&Footnote>::new());
    }
  }
}
