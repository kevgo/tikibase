/// a source reference on a Line
#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SourceReference {
    /// the textual identifier of the source
    pub identifier: String,
    /// where on the line the source reference starts
    pub start: u32,
    /// where on the line the source reference ends
    pub end: u32,
}
