//! The probes module contains the code that scans for problems in a TikiBase
//! and reports problems found as `Issues`.

pub(crate) mod image_orphaned;
pub(crate) mod link_broken;
pub(crate) mod section_capitalization;
pub(crate) mod section_duplicate;
pub(crate) mod section_empty;
pub(crate) mod section_no_header;
pub(crate) mod section_order;
pub(crate) mod section_type;
pub(crate) mod sources_missing;
