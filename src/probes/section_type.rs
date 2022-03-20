use crate::{config, Issue, Tikibase};

pub(crate) fn scan(base: &Tikibase, config: &config::Data) -> Vec<Issue> {
    let mut issues = Vec::<Issue>::new();
    let sections = match &config.sections {
        None => return issues,
        Some(sections) => sections,
    };
    for doc in &base.docs {
        for section in &doc.content_sections {
            let section_type = section.section_type();
            // HACK: see https://github.com/rust-lang/rust/issues/42671
            if !sections.iter().any(|s| s == section_type) {
                issues.push(Issue::UnknownSection {
                    file: doc.path.clone(),
                    line: section.line_number,
                    section_type: section_type.into(),
                    allowed_types: config.sections.clone().unwrap(),
                });
            }
        }
    }
    issues
}
