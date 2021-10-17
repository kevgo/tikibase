//! The issues module contains all possible issues
//! that Tikibase can identify and fix.

use crate::config;
use crate::database::Tikibase;

mod broken_image;
mod broken_link;
mod duplicate_section;
mod empty_section;
mod link_to_same_document;
mod link_without_destination;
mod missing_link;
mod missing_source;
mod mixed_section_capitalization;
mod obsolete_link;
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
pub(crate) use obsolete_link::ObsoleteLink;
pub(crate) use orphaned_resource::OrphanedResource;
pub(crate) use section_without_header::SectionWithoutHeader;
pub(crate) use unknown_section::UnknownSection;
pub(crate) use unordered_sections::UnorderedSections;

/// an issue that was identified in the Tikibase
pub trait Issue {
    /// fixes this issue, returns a human-readable description of what it did
    fn fix(&self, _base: &mut Tikibase, _config: &config::Data) -> String {
        unimplemented!()
    }

    /// indicates whether this issue is fixable
    fn fixable(&self) -> bool;
}
