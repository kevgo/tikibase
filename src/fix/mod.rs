//! Auto-fixing functionality

mod empty_section;
mod missing_links;
mod obsolete_occurrences_section;
mod unordered_sections;

use crate::{Config, Issue, Location, Tikibase};

/// fixes the given Issue
pub fn fix(issue: Issue, base: &mut Tikibase, config: &Config) -> Option<Fix> {
    match issue {
        // actual fixes
        Issue::EmptySection { location, title } => {
            Some(empty_section::remove_empty_section(base, title, location))
        }
        Issue::MissingLinks { location, links } => {
            Some(missing_links::add_occurrences(base, location, links))
        }
        Issue::ObsoleteOccurrencesSection { location } => Some(
            obsolete_occurrences_section::remove_occurrences_section(base, location),
        ),
        Issue::UnorderedSections { location } => Some(unordered_sections::sort_sections(
            base,
            location,
            config.sections.as_ref().unwrap(),
        )),
        // no-ops
        Issue::BrokenImage {
            location: _,
            target: _,
        }
        | Issue::BrokenLink {
            location: _,
            target: _,
        }
        | Issue::CannotReadConfigurationFile {
            message: _,
            location: _,
        }
        | Issue::DuplicateSection {
            location: _,
            title: _,
        }
        | Issue::InvalidConfigurationFile {
            message: _,
            location: _,
        }
        | Issue::MissingFootnote {
            location: _,
            identifier: _,
        }
        | Issue::MixCapSection {
            variants: _,
            location: _,
        }
        | Issue::LinkToSameDocument { location: _ }
        | Issue::LinkWithoutDestination { location: _ }
        | Issue::NoTitleSection { location: _ }
        | Issue::OrphanedResource { location: _ }
        | Issue::SectionWithoutHeader { location: _ }
        | Issue::UnclosedBacktick { location: _ }
        | Issue::UnclosedFence { location: _ }
        | Issue::UnknownSection {
            location: _,
            title: _,
            allowed_titles: _,
        }
        | Issue::UnusedFootnote {
            location: _,
            identifier: _,
        } => None,
    }
}

/// documents the fixes that this linter performs
pub enum Fix {
    AddedOccurrencesSection { location: Location },
    RemovedEmptySection { title: String, location: Location },
    RemovedObsoleteOccurrencesSection { location: Location },
    SortedSections { location: Location },
}
