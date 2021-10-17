use crate::config;
use crate::database::Tikibase;
use crate::Issue;
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;

/// describes an unknown section
pub struct UnknownSection {
    pub file: PathBuf,
    pub line: u32,
    pub section_type: String,
    pub allowed_types: Vec<String>,
}

impl Display for UnknownSection {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let alloweds: Vec<String> = self
            .allowed_types
            .iter()
            .map(|allowed| format!("\n  - {}", allowed))
            .collect();
        write!(
            f,
            "{}:{}  unknown section \"{}\", allowed sections:{}",
            self.file.to_string_lossy(),
            self.line + 1,
            self.section_type,
            alloweds.join("")
        )
    }
}

impl Issue for UnknownSection {
    fn fix(&self, _base: &mut Tikibase, _config: &config::Data) -> String {
        unimplemented!()
    }

    fn fixable(&self) -> bool {
        false
    }
}
