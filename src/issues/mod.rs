//! The issues module contains all possible issues
//! that Tikibase can identify and fix.

use crate::fixers::Fix;
use std::fmt::Display;

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

/// a problem that was identified in the Tikibase
pub(crate) trait Problem {
    /// if this problem is fixable, provides the Fixer implementation
    fn fixer(self: Box<Self>) -> Option<Box<dyn Fix>>;
}

/// This is the public type. It is a problem that can be displayed.
pub(crate) trait Issue: Problem + Display {}
// NOTE: this is necessary until https://github.com/rust-lang/rfcs/issues/2035 ships
impl<T> Issue for T where T: Problem + Display {}
