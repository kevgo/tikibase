use crate::Issue;

pub struct MissingSource {
    pub file: String,
    pub line: u32,
    pub index: String,
}

impl Issue for MissingSource {
    fn describe(&self) -> String {
        format!(
            "{}:{}  missing source [{}]",
            self.file,
            self.line + 1,
            self.index
        )
    }

    fn fixable(&self) -> bool {
        false
    }
}
