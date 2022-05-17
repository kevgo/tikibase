use super::Location;

/// the issues that this linter can find
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Issue {
    BrokenImage {
        location: Location,
        target: String,
    },
    CannotReadConfigurationFile {
        location: Location,
        message: String,
    },
    CannotReadDirectory {
        path: String,
        err: String,
    },
    CannotWriteConfigFile {
        message: String,
        file: String,
    },
    CannotWriteJsonSchemaFile {
        file: String,
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
        path: String,
    },
    EmptySection {
        location: Location,
        title: String,
    },
    HeadingLevelDifferentThanConfigured {
        location: Location,
        section_title: String,
        configured: u8,
        actual: u8,
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
        file: String,
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
        path: String,
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
        // This is a String and not a Path because we need a String (to print it),
        // and we already converted the Path of this orphaned resource into a String
        // during processing it.
        location: Location,
    },
    PathEscapesRoot {
        path: String,
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
