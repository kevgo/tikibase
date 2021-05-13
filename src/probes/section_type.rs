use super::{Issue, Issues};
use crate::config;
use crate::core::tikibase::Tikibase;
use std::path::PathBuf;

pub fn process(base: &Tikibase, config: &config::Data) -> Issues {
    let mut issues = Issues::new();
    let allowed_sections = match &config.allowed_sections {
        None => return issues,
        Some(allowed_sections) => allowed_sections,
    };
    for doc in &base.docs {
        for section in &doc.content_sections {
            let section_type = section.section_type();
            if !allowed_sections.contains(&section_type) {
                issues.push(Box::new(UnknownSection {
                    file: doc.path.clone(),
                    line: section.line_number,
                    section_type: section_type,
                    allowed_types: config.allowed_sections.clone().unwrap(),
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
        format!(
            "{}:{}  unknown section \"{}\", allowed sections: {}",
            self.file.to_string_lossy(),
            self.line + 1,
            self.section_type,
            self.allowed_types.join(" | ")
        )
    }

    fn fix(&self, _base: &mut Tikibase) -> String {
        panic!("not fixable")
    }

    fn fixable(&self) -> bool {
        false
    }
}
