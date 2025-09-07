use crate::domain::PathRelativeToRoot;

use super::Issue;
use super::scanners::{section_capitalization, section_level};
use ahash::AHashMap;

/// state for phase 2
pub struct State2 {
  pub capitalization_outliers: AHashMap<String, section_capitalization::OutlierInfo>,
  pub level_outliers: AHashMap<String, section_level::OutlierInfo>,
  pub linked_resources: Vec<PathRelativeToRoot>,
  pub issues: Vec<Issue>,
}
