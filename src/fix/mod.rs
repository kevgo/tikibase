//! Auto-fixing functionality

mod empty_section;
mod missing_links;
mod obsolete_occurrences_section;
mod unordered_sections;

use crate::{Config, Issue, Location, Tikibase};

/// fixes the given Issue
pub fn fix(issue: Issue, base: &mut Tikibase, config: &Config) -> Option<Fix> {
    match issue {
        Issue::BrokenImage {
            location: _,
            target: _,
        } => None,
        Issue::BrokenLink {
            location: _,
            target: _,
        } => None,
        Issue::CannotReadConfigurationFile {
            message: _,
            location: _,
        } => None,
        Issue::DuplicateSection {
            location: _,
            title: _,
        } => None,
        Issue::EmptySection {
            location,
            section_type,
        } => Some(empty_section::remove_empty_section(
            base,
            section_type,
            location,
        )),
        Issue::InvalidConfigurationFile {
            message: _,
            location: _,
        } => None,
        Issue::LinkToSameDocument { location: _ } => None,
        Issue::LinkWithoutDestination { location: _ } => None,
        Issue::MissingLinks { location, links } => {
            Some(missing_links::add_occurrences(base, location, links))
        }
        Issue::MissingSource {
            location: _,
            index: _,
        } => None,
        Issue::MixCapSection {
            variants: _,
            location: _,
        } => None,
        Issue::NoTitleSection { location: _ } => None,
        Issue::ObsoleteOccurrencesSection { location } => Some(
            obsolete_occurrences_section::remove_occurrences_section(base, location),
        ),
        Issue::OrphanedResource { location: _ } => None,
        Issue::SectionWithoutHeader { location: _ } => None,
        Issue::UnclosedFence { location: _ } => None,
        Issue::UnknownSection {
            location: _,
            section_type: _,
            allowed_types: _,
        } => None,
        Issue::UnorderedSections { location } => Some(unordered_sections::sort_sections(
            base,
            location,
            config.sections.as_ref().unwrap(),
        )),
    }
}

/// documents the fixes that this linter performs
pub enum Fix {
    AddedOccurrencesSection {
        location: Location,
    },
    RemovedEmptySection {
        section_type: String,
        location: Location,
    },
    RemovedObsoleteOccurrencesSection {
        location: Location,
    },
    SortedSections {
        location: Location,
    },
}
