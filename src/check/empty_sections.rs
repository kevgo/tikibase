use crate::core::tikibase::Tikibase;
use std::path::PathBuf;

pub struct EmptySection {
    pub path: PathBuf,
    pub line: u32,
}

/// finds empty sections
pub fn find(base: &Tikibase) -> Vec<EmptySection> {
    let mut result = Vec::new();
    for doc in &base.docs {
        for section in &doc.content_sections {
            if section.body.is_empty() {
                result.push(EmptySection {
                    path: doc.path.clone(),
                    line: section.line_number,
                });
            }
        }
    }
    result
}
