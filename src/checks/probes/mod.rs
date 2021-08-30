//! The probes module contains the code that scans for problems in a TikiBase
//! and reports problems found as `Issues`.

pub mod image_orphaned;
pub mod link_broken;
pub mod section_capitalization;
pub mod section_duplicate;
pub mod section_empty;
pub mod section_no_header;
pub mod section_order;
pub mod section_type;
pub mod sources_missing;
