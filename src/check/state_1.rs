use super::Issue;
use ahash::AHashMap;
use std::path::PathBuf;

/// mutable state accumulated in check phase 1
pub struct State1 {
    pub issues: Vec<Issue>,
    pub linked_resources: Vec<PathBuf>,
    pub capitalization_variants: AHashMap<String, u32>,
    pub level_variants: AHashMap<String, AHashMap<u8, u32>>,
}
