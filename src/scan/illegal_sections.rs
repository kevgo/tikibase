use crate::database::Section;
use crate::{Config, Issue, Location};
use std::path::Path;

/// populates the given issues list with all sections in this document that don't match the configured sections
pub fn scan(section: &Section, path: &Path, config: &Config, issues: &mut Vec<Issue>) {
    let section_title = section.human_title();
    if !config.matching_title(section_title) {
        issues.push(Issue::UnknownSection {
            location: Location {
                file: path.into(),
                line: section.line_number,
                start: section.title_text_start as u32,
                end: section.title_text_end(),
            },
            title: section_title.into(),
            allowed_titles: config.sections.clone().unwrap(),
        });
    }
}
