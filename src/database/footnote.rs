/// reference to a footnote
#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FootnoteReference {
    /// the textual identifier of the source
    pub identifier: String,
    /// the line on which this footnote definition exists
    pub line: u32,
    /// where on the line the source reference starts
    pub start: u32,
    /// where on the line the source reference ends
    pub end: u32,
}

/// definition of a footnote
#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FootnoteDefinition {
    /// the textual identifier of the source
    pub identifier: String,
    /// the line on which this footnote definition exists
    pub line: u32,
    /// where on the line the source reference starts
    pub start: u32,
    /// where on the line the source reference ends
    pub end: u32,
}
