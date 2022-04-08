//! Auto-fixing functionality

mod empty_section;
mod missing_links;
mod obsolete_occurrences_section;
mod unordered_sections;

use crate::{Config, Issue, Location, Tikibase};

/// fixes the given Issue
pub fn fix(issue: Issue, base: &mut Tikibase, config: &Config) -> FixResult {
    match issue {
        // actual fixes
        Issue::EmptySection { location, title } => {
            empty_section::remove_section(base, title, location)
        }
        Issue::MissingLinks { location, links } => {
            missing_links::add_occurrences(base, location, links, config)
        }
        Issue::ObsoleteOccurrencesSection { location } => {
            obsolete_occurrences_section::remove_occurrences_section(base, location)
        }
        Issue::UnorderedSections { location } => {
            unordered_sections::sort_sections(base, location, config.sections.as_ref().unwrap())
        }
        // no-ops
        Issue::BrokenImage {
            location: _,
            target: _,
        }
        | Issue::CannotWriteConfigFile {
            file: _,
            message: _,
        }
        | Issue::CannotWriteJsonSchemaFile {
            file: _,
            message: _,
        }
        | Issue::LinkToNonExistingFile {
            location: _,
            target: _,
        }
        | Issue::CannotReadConfigurationFile {
            message: _,
            location: _,
        }
        | Issue::DocumentWithoutLinks { location: _ }
        | Issue::DuplicateSection {
            location: _,
            title: _,
        }
        | Issue::InvalidConfigurationFile {
            message: _,
            location: _,
        }
        | Issue::InvalidGlob {
            glob: _,
            location: _,
            message: _,
        }
        | Issue::InvalidTitleRegex { regex: _, file: _ }
        | Issue::LinkToNonExistingAnchorInCurrentDocument {
            location: _,
            anchor: _,
        }
        | Issue::LinkToNonExistingAnchorInExistingDocument {
            location: _,
            target_file: _,
            anchor: _,
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
        | Issue::LinkWithoutTarget { location: _ }
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
        } => FixResult::Unfixable,
    }
}

/// documents the fixes that this linter performs
pub enum Fix {
    AddedOccurrencesSection { location: Location },
    RemovedEmptySection { title: String, location: Location },
    RemovedObsoleteOccurrencesSection { location: Location },
    SortedSections { location: Location },
}

/// result of a fix operation
pub enum FixResult {
    /// the issue was fixed
    Fixed(Fix),
    /// the given Issue occurred while trying to fix this issue
    Failed(Issue),
    /// this issue is not fixable
    Unfixable,
}

impl FixResult {
    pub fn fixed(fix: Fix) -> FixResult {
        FixResult::Fixed(fix)
    }
}
