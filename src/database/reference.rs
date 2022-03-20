/// a link in the document
#[derive(Debug, PartialEq)]
pub enum Reference {
    Link { destination: String },
    Image { src: String },
}
