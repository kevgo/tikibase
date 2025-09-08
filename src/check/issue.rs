use crate::domain::PathRelativeToRoot;

use super::Location;

/// the issues that this linter can find
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Issue {
  BrokenImage {
    location: Location,
    target: String,
  },
  // TODO: make error
  CannotReadConfigurationFile {
    location: Location,
    message: String,
  },
  // TODO: make error
  CannotReadDirectory {
    path: PathRelativeToRoot,
    err: String,
  },
  // TODO: make error
  CannotWriteConfigFile {
    message: String,
    file: PathRelativeToRoot,
  },
  // TODO: make error
  CannotWriteJsonSchemaFile {
    file: PathRelativeToRoot,
    message: String,
  },
  DocumentWithoutLinks {
    location: Location,
  },
  DuplicateSection {
    location: Location,
    title: String,
  },
  EmptyDocument {
    path: PathRelativeToRoot,
  },
  EmptySection {
    location: Location,
    title: String,
  },
  HeadingLevelDifferentThanConfigured {
    location: Location,
    configured_level: u8,
    configured_title: String,
    actual_level: u8,
    actual_title: String,
  },
  InconsistentHeadingLevel {
    location: Location,
    /// human-readable section title
    section_title: String,
    /// the most commonly observed level (if one exists)
    common_level: Option<u8>,
    /// the level used here
    this_level: u8,
    /// all observed variants
    all_levels: Vec<u8>,
  },
  InvalidConfigurationFile {
    location: Location,
    message: String,
  },
  InvalidGlob {
    glob: String,
    message: String,
    location: Location,
  },
  InvalidTitleRegex {
    regex: String,
    problem: String,
    file: PathRelativeToRoot,
  },
  LinkToNonExistingAnchorInCurrentDocument {
    location: Location,
    /// the non-existing anchor in the current
    anchor: String,
  },
  LinkToNonExistingAnchorInExistingDocument {
    location: Location,
    /// the file that the link points to
    target_file: String,
    /// the non-existing anchor in that file
    anchor: String,
  },
  LinkToNonExistingDir {
    location: Location,
    target: String,
  },
  LinkToNonExistingFile {
    location: Location,
    target: String,
  },
  LinkToSameDocument {
    location: Location,
  },
  LinkWithoutTarget {
    location: Location,
  },
  MissingFootnote {
    location: Location,
    identifier: String,
  },
  MissingLink {
    location: Location,
    path: PathRelativeToRoot,
    title: String,
  },
  MixCapSection {
    location: Location,
    all_variants: Vec<String>,
    this_variant: String,
    common_variant: Option<String>,
    section_level: u8,
  },
  NoTitleSection {
    location: Location,
  },
  ObsoleteOccurrencesSection {
    location: Location,
  },
  OrphanedResource {
    location: Location,
  },
  PathEscapesRoot {
    path: PathRelativeToRoot,
    location: Location,
  },
  SectionWithoutHeader {
    location: Location,
  },
  TitleRegexNoCaptures {
    regex: String,
  },
  TitleRegexTooManyCaptures {
    regex: String,
    captures: usize,
  },
  UnclosedBacktick {
    location: Location,
  },
  UnclosedFence {
    location: Location,
  },
  UnknownSection {
    location: Location,
    title: String,
    allowed_titles: Vec<String>,
  },
  UnorderedSections {
    location: Location,
  },
  UnusedFootnote {
    location: Location,
    identifier: String,
  },
}
