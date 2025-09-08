//! Auto-fixing functionality

mod empty_section;
mod inconsistent_levels;
mod missing_links;
mod mix_cap_section;
mod obsolete_occurrences_section;
mod unordered_sections;

use crate::Tikibase;
use crate::check::{Issue, Location};

/// fixes the given Issue
pub fn fix(issue: Issue, base: &mut Tikibase) -> Result {
  match issue {
    // actual fixes
    Issue::EmptySection { location, title } => empty_section::remove_section(base, title, location),
    Issue::HeadingLevelDifferentThanConfigured {
      location,
      configured_level,
      configured_title,
      actual_level,
      actual_title,
    } => inconsistent_levels::set_to_configured_section_level(
      base,
      location,
      actual_level,
      actual_title,
      configured_level,
      configured_title,
    ),
    Issue::InconsistentHeadingLevel {
      location,
      common_level,
      this_level,
      section_title,
      all_levels: _,
    } => {
      if let Some(common_level) = common_level {
        inconsistent_levels::normalize_outliers(
          base,
          location,
          section_title,
          this_level,
          common_level,
        )
      } else {
        Result::Unfixable
      }
    }
    Issue::MissingLink {
      location,
      path,
      title,
    } => missing_links::add_occurrences(base, location, path, &title),
    Issue::MixCapSection {
      location,
      all_variants: _,
      section_level,
      this_variant,
      common_variant,
    } => {
      if let Some(common_variant) = common_variant {
        mix_cap_section::normalize_capitalization(
          base,
          location,
          section_level,
          this_variant,
          common_variant,
        )
      } else {
        Result::Unfixable
      }
    }
    Issue::ObsoleteOccurrencesSection { location } => {
      obsolete_occurrences_section::remove_occurrences_section(base, location)
    }
    Issue::UnorderedSections { location } => unordered_sections::sort_sections(base, location),
    // no-ops
    Issue::BrokenImage {
      location: _,
      target: _,
    }
    | Issue::CannotReadDirectory { path: _, err: _ }
    | Issue::CannotWriteConfigFile {
      file: _,
      message: _,
    }
    | Issue::CannotWriteJsonSchemaFile {
      file: _,
      message: _,
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
    | Issue::EmptyDocument { path: _ }
    | Issue::InvalidConfigurationFile {
      message: _,
      location: _,
    }
    | Issue::InvalidGlob {
      glob: _,
      location: _,
      message: _,
    }
    | Issue::InvalidTitleRegex {
      regex: _,
      problem: _,
      file: _,
    }
    | Issue::LinkToNonExistingAnchorInCurrentDocument {
      location: _,
      anchor: _,
    }
    | Issue::LinkToNonExistingAnchorInExistingDocument {
      location: _,
      target_file: _,
      anchor: _,
    }
    | Issue::LinkToNonExistingFile {
      location: _,
      target: _,
    }
    | Issue::LinkToNonExistingDir {
      location: _,
      target: _,
    }
    | Issue::MissingFootnote {
      location: _,
      identifier: _,
    }
    | Issue::LinkToSameDocument { location: _ }
    | Issue::LinkWithoutTarget { location: _ }
    | Issue::NoTitleSection { location: _ }
    | Issue::OrphanedResource { location: _ }
    | Issue::SectionWithoutHeader { location: _ }
    | Issue::TitleRegexNoCaptures { regex: _ }
    | Issue::TitleRegexTooManyCaptures {
      regex: _,
      captures: _,
    }
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
    } => Result::Unfixable,
  }
}

/// documents the fixes that this linter performs
#[derive(Debug, Eq, PartialEq)]
pub enum Fix {
  AddedOccurrencesSection {
    location: Location,
    target: String,
  },
  NormalizedSectionCapitalization {
    location: Location,
    old_capitalization: String,
    new_capitalization: String,
  },
  NormalizedSectionLevel {
    location: Location,
    section_human_title: String,
    old_level: u8,
    new_level: u8,
  },
  RemovedEmptySection {
    title: String,
    location: Location,
  },
  RemovedObsoleteOccurrencesSection {
    location: Location,
  },
  SortedSections {
    location: Location,
  },
}

/// result of a fix operation
pub enum Result {
  /// the issue was fixed
  Fixed(Fix),
  /// the given Issue occurred while trying to fix this issue
  Failed(Issue),
  /// this issue is not fixable
  Unfixable,
}
