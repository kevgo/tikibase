/// a link in the document
pub enum Reference {
    Link { destination: String },
    Image { src: String },
}
