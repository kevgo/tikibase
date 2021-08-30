//! The issues module contains the issues that Tikibase can find.

mod broken_image;
mod broken_link;
mod duplicate_section;
mod empty_section;
mod link_to_same_document;
mod link_without_destination;
mod missing_source;
mod mixed_section_capitalization;
mod orphaned_resource;
mod section_without_header;
mod unknown_section;
mod unordered_sections;

pub use broken_image::BrokenImage;
pub use broken_link::BrokenLink;
pub use duplicate_section::DuplicateSection;
pub use empty_section::EmptySection;
pub use link_to_same_document::LinkToSameDocument;
pub use link_without_destination::LinkWithoutDestination;
pub use missing_source::MissingSource;
pub use mixed_section_capitalization::MixCapSection;
pub use orphaned_resource::OrphanedResource;
pub use section_without_header::SectionNoHeader;
pub use unknown_section::UnknownSection;
pub use unordered_sections::UnorderedSections;
