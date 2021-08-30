pub mod broken_image;
pub mod broken_link;
pub mod duplicate_section;
pub mod link_to_same_document;
pub mod link_without_destination;
pub mod mixed_section_capitalization;
pub mod orphaned_resource;

pub use broken_image::BrokenImage;
pub use broken_link::BrokenLink;
pub use duplicate_section::DuplicateSection;
pub use link_to_same_document::LinkToSameDocument;
pub use link_without_destination::LinkWithoutDestination;
pub use mixed_section_capitalization::MixCapSection;
pub use orphaned_resource::OrphanedResource;
