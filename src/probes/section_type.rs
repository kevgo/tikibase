use super::{Issue, Issues};
use crate::config;
use crate::core::tikibase::Tikibase;
use std::path::PathBuf;

pub fn process(base: &Tikibase, config: &config::Data) -> Issues {
    let mut issues = Issues::new();
    let sections = match &config.sections {
        None => return issues,
        Some(sections) => sections,
    };
    for doc in &base.docs {
        for section in &doc.content_sections {
            let section_type = section.section_type();
            let contains = sections.iter().any(|s| s == section_type); // see https://github.com/rust-lang/rust/issues/42671
            if !contains {
                issues.push(Box::new(UnknownSection {
                    file: doc.path.clone(),
                    line: section.line_number,
                    section_type: section_type.to_string(),
                    allowed_types: config.sections.clone().unwrap(),
                }));
            }
        }
    }
    issues
}

/// describes an unknown section
struct UnknownSection {
    file: PathBuf,
    line: u32,
    section_type: String,
    allowed_types: Vec<String>,
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

// TODO: tests
