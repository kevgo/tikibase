use super::scanners::{section_capitalization, section_level};
use super::Issue;
use ahash::AHashMap;

/// state for phase 2
pub struct State2 {
  pub capitalization_outliers: AHashMap<String, section_capitalization::OutlierInfo>,
  pub level_outliers: AHashMap<String, section_level::OutlierInfo>,
  pub linked_resources: Vec<String>,
  pub issues: Vec<Issue>,
}
