use std::path::PathBuf;

use crate::config;
use crate::core::tikibase::Tikibase;

use super::{Issue, Issues};

pub fn process(base: &Tikibase, config: &config::Data) -> Issues {
    let mut issues = Issues::new();
    for doc in &base.docs {
        for section in &doc.content_sections {
            let section_type = section.section_type();
            match &config.allowed_sections {
                None => return issues,
                Some(allowed_sections) => {
                    if !allowed_sections.contains(&section_type) {
                        issues.push(Box::new(UnknownSection {
                            file: doc.path.clone(),
                            line: section.line_number,
                            section_type: section_type,
                        }));
                    }
                }
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
}

impl Issue for UnknownSection {
    fn describe(&self) -> String {
        format!(
            "{}:{}  unknown section \"{}\"",
            self.file.to_string_lossy(),
            self.line,
            self.section_type
        )
    }

    fn fix(&self, _base: &mut Tikibase) -> String {
        panic!("not fixable")
    }

    fn fixable(&self) -> bool {
        false
    }
}
