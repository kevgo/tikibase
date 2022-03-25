/// a link in the document
#[derive(Debug, PartialEq)]
pub enum Reference {
    Link {
        destination: String,
        line: u32,
        start: u32,
        end: u32,
    },
    Image {
        src: String,
        line: u32,
        start: u32,
        end: u32,
    },
}
