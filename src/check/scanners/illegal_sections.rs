use crate::check::{Issue, Location};
use crate::database::Section;
use crate::Config;

/// populates the given issues list with all sections in this document that don't match the configured sections
pub fn scan(section: &Section, path: &str, config: &Config, issues: &mut Vec<Issue>) {
    if !config.matching_title(&section.title_line.text) {
        let (actual_level, _start) = Section::parse_title(&section.title_line.text);
        match config.section_with_human_title(section.human_title()) {
            Some(configured_title) => {
                let (configured_level, _start) = Section::parse_title(configured_title);
                issues.push(Issue::HeadingLevelDifferentThanConfigured {
                    location: Location {
                        file: path.into(),
                        line: section.line_number,
                        start: section.title_text_start as u32,
                        end: section.title_text_end(),
                    },
                    section_title: section.title_line.text.clone(),
                    configured: configured_level,
                    actual: actual_level,
                });
            }
            None => {
                issues.push(Issue::UnknownSection {
                    location: Location {
                        file: path.into(),
                        line: section.line_number,
                        start: section.title_text_start as u32,
                        end: section.title_text_end(),
                    },
                    title: section.title_line.text.clone(),
                    allowed_titles: config.sections.clone().unwrap(),
                });
            }
        }
    }
}
