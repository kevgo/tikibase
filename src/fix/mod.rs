//! Auto-fixing functionality

mod empty_section;
mod missing_links;
mod obsolete_occurrences_section;
mod unordered_sections;

use crate::{Config, Issue, Position, Tikibase};

/// fixes the given Issue
pub fn fix(issue: Issue, base: &mut Tikibase, config: &Config) -> Option<Fix> {
    match issue {
        Issue::BrokenImage { pos: _, target: _ } => None,
        Issue::BrokenLink { pos: _, target: _ } => None,
        Issue::CannotReadConfigurationFile { message: _, pos: _ } => None,
        Issue::DuplicateSection {
            pos: _,
            section_type: _,
        } => None,
        Issue::EmptySection { pos, section_type } => {
            Some(empty_section::remove_empty_section(base, section_type, pos))
        }
        Issue::InvalidConfigurationFile { message: _, pos: _ } => None,
        Issue::LinkToSameDocument { pos: _ } => None,
        Issue::LinkWithoutDestination { pos: _ } => None,
        Issue::MissingLinks { pos, links } => {
            Some(missing_links::add_occurrences(base, pos, links))
        }
        Issue::MissingSource { pos: _, index: _ } => None,
        Issue::MixCapSection {
            variants: _,
            pos: _,
        } => None,
        Issue::NoTitleSection { pos: _ } => None,
        Issue::ObsoleteOccurrencesSection { pos } => Some(
            obsolete_occurrences_section::remove_occurrences_section(base, pos),
        ),
        Issue::OrphanedResource { pos: _ } => None,
        Issue::SectionWithoutHeader { pos: _ } => None,
        Issue::UnclosedFence { pos: _ } => None,
        Issue::UnknownSection {
            pos: _,
            section_type: _,
            allowed_types: _,
        } => None,
        Issue::UnorderedSections { pos } => Some(unordered_sections::sort_sections(
            base,
            pos,
            config.sections.as_ref().unwrap(),
        )),
    }
}

/// documents the fixes that this linter performs
pub enum Fix {
    AddedOccurrencesSection { pos: Position },
    RemovedEmptySection { section_type: String, pos: Position },
    RemovedObsoleteOccurrencesSection { pos: Position },
    SortedSections { pos: Position },
}
