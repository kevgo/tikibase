mod empty_section;
mod missing_links;
mod obsolete_occurrences_section;
mod unordered_sections;

use super::config;
use crate::{Issue, Tikibase};
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;

/// fixes the given Issue
pub fn fix(issue: Issue, base: &mut Tikibase, config: &config::Data) -> Option<Fix> {
    match issue {
        Issue::BrokenImage {
            file: _,
            line: _,
            target: _,
        } => None,
        Issue::BrokenLink {
            file: _,
            line: _,
            target: _,
        } => None,
        Issue::CannotReadConfigurationFile { message: _ } => None,
        Issue::DuplicateSection {
            file: _,
            section_type: _,
        } => None,
        Issue::EmptySection {
            file: filename,
            line,
            section_type,
        } => Some(empty_section::remove_empty_section(
            base,
            section_type,
            filename,
            line,
        )),
        Issue::InvalidConfigurationFile { message: _ } => None,
        Issue::LinkToSameDocument { file: _, line: _ } => None,
        Issue::LinkWithoutDestination { file: _, line: _ } => None,
        Issue::MissingLinks { file, links } => {
            Some(missing_links::add_occurrences(base, file, links))
        }
        Issue::MissingSource {
            file: _,
            line: _,
            index: _,
        } => None,
        Issue::MixCapSection { variants: _ } => None,
        Issue::NoTitleSection { file: _ } => None,
        Issue::ObsoleteOccurrencesSection { file, line } => Some(
            obsolete_occurrences_section::remove_occurrences_section(base, file, line),
        ),
        Issue::OrphanedResource { path: _ } => None,
        Issue::SectionWithoutHeader { file: _, line: _ } => None,
        Issue::UnclosedFence { file: _, line: _ } => None,
        Issue::UnknownSection {
            file: _,
            line: _,
            section_type: _,
            allowed_types: _,
        } => None,
        Issue::UnorderedSections { file } => Some(unordered_sections::sort_sections(
            base,
            file,
            config.sections.as_ref().unwrap(),
        )),
    }
}

/// documents the fixes that this linter performs
pub enum Fix {
    AddedOccurrencesSection {
        file: PathBuf,
        line: u32,
    },
    RemovedEmptySection {
        section_type: String,
        filename: PathBuf,
        line: u32,
    },
    RemovedObsoleteOccurrencesSection {
        file: PathBuf,
        line: u32,
    },
    SortedSections {
        file: PathBuf,
    },
}

impl Display for Fix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Fix::RemovedEmptySection {
                section_type,
                filename,
                line,
            } => write!(
                f,
                "{}:{}  removed empty section \"{}\"",
                filename.to_string_lossy(),
                line + 1,
                section_type
            ),
            Fix::AddedOccurrencesSection { file, line } => write!(
                f,
                "{}:{}  added occurrences section",
                file.to_string_lossy(),
                line
            ),
            Fix::RemovedObsoleteOccurrencesSection { file, line } => write!(
                f,
                "{}:{}  removed obsolete occurrences section",
                file.to_string_lossy(),
                line + 1,
            ),
            Fix::SortedSections { file } => {
                write!(f, "{}  fixed section order", file.to_string_lossy())
            }
        }
    }
}
