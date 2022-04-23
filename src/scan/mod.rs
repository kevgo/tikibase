//! The probes module contains the code that scans for problems in a `TikiBase`
//! and reports problems found as `Issues`.

pub(crate) mod footnotes;
pub(crate) mod image_orphaned;
pub(crate) mod links;
pub(crate) mod occurrences;
pub(crate) mod section_capitalization;
pub(crate) mod section_level;
pub(crate) mod section_without_header;
