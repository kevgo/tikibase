use super::Issue;
use crate::database::Directory;
use crate::domain::PathRelativeToRoot;
use ahash::AHashMap;

/// mutable state that gets accumulated in phase 1 of the check process
pub struct State1<'a> {
  /// issues found in phase 1
  pub issues: Vec<Issue>,
  /// relative path of resource files to which links exist
  pub linked_resources: Vec<PathRelativeToRoot>,
  /// capitalization variant --> how often this variant occurs
  pub capitalization_variants: AHashMap<String, u32>,
  /// section title --> indentation level --> how often this title occurs with that indentation
  pub level_variants: AHashMap<String, AHashMap<u8, u32>>,
  /// link to the root directory of the Tikibase
  pub base_dir: &'a Directory,
}

impl<'a> State1<'a> {
  pub fn empty(base_dir: &'a Directory) -> State1<'a> {
    State1 {
      issues: vec![],
      linked_resources: vec![],
      capitalization_variants: AHashMap::new(),
      level_variants: AHashMap::new(),
      base_dir,
    }
  }
}
