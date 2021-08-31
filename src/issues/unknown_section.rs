use crate::config;
use crate::database::Tikibase;
use crate::Issue;
use std::path::PathBuf;

/// describes an unknown section
pub struct UnknownSection {
    pub file: PathBuf,
    pub line: u32,
    pub section_type: String,
    pub allowed_types: Vec<String>,
}

impl Issue for UnknownSection {
    fn describe(&self) -> String {
        let alloweds: Vec<String> = self
            .allowed_types
            .iter()
            .map(|allowed| format!("\n  - {}", allowed))
            .collect();
        format!(
            "{}:{}  unknown section \"{}\", allowed sections:{}",
            self.file.to_string_lossy(),
            self.line + 1,
            self.section_type,
            alloweds.join("")
        )
    }

    fn fix(&self, _base: &mut Tikibase, _config: &config::Data) -> String {
        panic!("not fixable")
    }

    fn fixable(&self) -> bool {
        false
    }
}
