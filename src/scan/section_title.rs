use crate::{Config, Issue, Location, Tikibase};

pub(crate) fn scan(base: &Tikibase, config: &Config) -> Vec<Issue> {
    let mut issues = Vec::<Issue>::new();
    let config_sections = match &config.sections {
        None => return issues,
        Some(sections) => sections,
    };
    for doc in &base.docs {
        for section in &doc.content_sections {
            let section_title = section.title();
            // HACK: see https://github.com/rust-lang/rust/issues/42671
            if !config_sections
                .iter()
                .any(|config_section| config_section == section_title.title)
            {
                issues.push(Issue::UnknownSection {
                    location: Location {
                        file: doc.path.clone(),
                        line: section.line_number,
                        start: section_title.start,
                        end: section_title.end(),
                    },
                    title: section_title.title.into(),
                    allowed_titles: config.sections.clone().unwrap(),
                });
            }
        }
    }
    issues
}
