//! The issues module contains the issues that Tikibase can find.

mod broken_image;
mod broken_link;
mod duplicate_section;
mod empty_section;
mod link_to_same_document;
mod link_without_destination;
mod missing_link;
mod missing_source;
mod mixed_section_capitalization;
mod orphaned_resource;
mod section_without_header;
mod unknown_section;
mod unordered_sections;

pub(crate) use broken_image::BrokenImage;
pub(crate) use broken_link::BrokenLink;
pub(crate) use duplicate_section::DuplicateSection;
pub(crate) use empty_section::EmptySection;
pub(crate) use link_to_same_document::LinkToSameDocument;
pub(crate) use link_without_destination::LinkWithoutDestination;
pub(crate) use missing_link::{MissingLink, MissingLinks};
pub(crate) use missing_source::MissingSource;
pub(crate) use mixed_section_capitalization::MixCapSection;
pub(crate) use orphaned_resource::OrphanedResource;
pub(crate) use section_without_header::SectionNoHeader;
pub(crate) use unknown_section::UnknownSection;
pub(crate) use unordered_sections::UnorderedSections;
