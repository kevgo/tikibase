/// a link in the document
#[derive(Debug, PartialEq)]
pub enum Reference {
    Link {
        destination: String,
        start: u32,
        end: u32,
    },
    Image {
        src: String,
        start: u32,
        end: u32,
    },
}
