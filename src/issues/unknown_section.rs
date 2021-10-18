use super::Problem;
use crate::fixers::Fix;
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

impl Problem for UnknownSection {
    fn fixer(&self) -> Option<Box<dyn Fix>> {
        None
    }
}
