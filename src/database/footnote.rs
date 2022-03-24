#[derive(Debug, Default, PartialEq)]
pub struct Footnotes {
    pub definitions: Vec<Footnote>,
    pub references: Vec<Footnote>,
}

impl Footnotes {
    pub fn missing_references(&self) -> Vec<&Footnote> {
        let mut result = vec![];
        for reference in &self.references {
            if self
                .definitions
                .iter()
                .any(|definition| definition.identifier == reference.identifier)
            {
                result.push(reference);
            }
        }
        result
    }

    pub fn unused_definitions(&self) -> Vec<&Footnote> {
        let mut result = vec![];
        for definition in &self.definitions {
            if self
                .references
                .iter()
                .any(|reference| reference.identifier == definition.identifier)
            {
                result.push(definition);
            }
        }
        result
    }
}

/// reference to a footnote
#[derive(Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Footnote {
    /// the textual identifier of the source
    pub identifier: String,
    /// the line on which this footnote definition exists
    pub line: u32,
    /// where on the line the source reference starts
    pub start: u32,
    /// where on the line the source reference ends
    pub end: u32,
}

#[cfg(test)]
mod tests {

    mod missing_references {
        use crate::database::{Footnote, Footnotes};

        #[test]
        fn has_missing() {
            let give = Footnotes {
                definitions: vec![
                    Footnote {
                        identifier: "f1".into(),
                        ..Footnote::default()
                    },
                    Footnote {
                        identifier: "f2".into(),
                        ..Footnote::default()
                    },
                ],
                references: vec![Footnote {
                    identifier: "f2".into(),
                    ..Footnote::default()
                }],
            };
            let have = give.missing_references();
            let want1 = Footnote {
                identifier: "f1".into(),
                ..Footnote::default()
            };
            pretty::assert_eq!(have, vec![&want1]);
        }
    }
}
