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
pub(crate) use section_without_header::SectionNoHeader;
pub(crate) use unknown_section::UnknownSection;
pub(crate) use unordered_sections::UnorderedSections;

use crate::database::{config, Tikibase};
use std::slice::Iter;

/// an issue that was identified in the Tikibase
pub trait Issue {
    /// provides a human-readable description of the issue
    fn describe(&self) -> String;

    /// fixes this issue, returns a human-readable description of what it did
    fn fix(&self, base: &mut Tikibase, config: &config::Data) -> String;

    /// indicates whether this issue is fixable
    fn fixable(&self) -> bool;
}

/// a collection of issues
pub struct Issues(Vec<Box<dyn Issue>>);

impl Issues {
    /// appends the given issue to this issue collection
    pub fn extend(&mut self, new_issues: Issues) {
        self.0.extend(new_issues.0);
    }

    /// provides an iterator over borrowed references to the contained Issues
    pub fn iter(&self) -> Iter<Box<dyn Issue>> {
        self.0.iter()
    }

    #[cfg(test)]
    /// indicates whether this collection contains any elements
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// provides an empty Issues instance
    pub fn new() -> Self {
        Issues(Vec::new())
    }

    /// adds the given Issue to this Issue collection
    pub fn push(&mut self, issue: Box<dyn Issue>) {
        self.0.push(issue);
    }
}

impl Default for Issues {
    fn default() -> Self {
        Issues::new()
    }
}

impl IntoIterator for Issues {
    type Item = Box<dyn Issue>;

    type IntoIter = std::vec::IntoIter<Box<dyn Issue>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
