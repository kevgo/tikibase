use crate::config;
use crate::database::Tikibase;
use crate::{issues, Issues};

pub fn process(base: &Tikibase, config: &config::Data) -> Issues {
    let mut issues = Issues::new();
    let sections = match &config.sections {
        None => return issues,
        Some(sections) => sections,
    };
    for doc in &base.docs {
        for section in &doc.content_sections {
            let section_type = section.section_type();
            // HACK: see https://github.com/rust-lang/rust/issues/42671
            if !sections.iter().any(|s| s == section_type) {
                issues.push(Box::new(issues::UnknownSection {
                    file: doc.path.clone(),
                    line: section.line_number,
                    section_type: section_type.into(),
                    allowed_types: config.sections.clone().unwrap(),
                }));
            }
        }
    }
    issues
}
