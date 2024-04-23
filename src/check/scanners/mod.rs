//! The probes module contains the code that scans for problems in a `TikiBase`
//! and reports problems found as `Issues`.

pub mod duplicate_sections;
pub mod empty_section_content;
pub mod empty_section_title;
pub mod footnotes;
pub mod illegal_sections;
pub mod links;
pub mod obsolete_occurrences;
pub mod orphaned_resource;
pub mod section_capitalization;
pub mod section_level;
pub mod unordered_sections;
