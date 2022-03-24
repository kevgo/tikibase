#[derive(Debug, Default, PartialEq)]
pub struct Footnotes {
    pub definitions: Vec<Footnote>,
    pub references: Vec<Footnote>,
}

impl Footnotes {
    /// consumes the given Footnotes and adds all their content to self
    pub fn append(&mut self, other: Footnotes) {
        self.definitions.append(&mut other.definitions);
        self.references.append(&mut other.references);
    }
    fn missing_references(&self) -> Vec<&Footnote> {
        self.definitions
            .iter()
            .any(|definition| definition.identifier == identifier)
    }

    fn unused_definitions(references: &[FootnoteReference], identifier: &str) -> bool {
        references
            .iter()
            .any(|reference| reference.identifier == identifier)
    }
}

/// reference to a footnote
#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
