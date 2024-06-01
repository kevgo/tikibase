mod empty_section_content;

use crate::database::Tikibase;

// a problem that was identified in the given document collection
pub trait Issue {
  // provides an end-user facing description of the problem
  fn describe(&self) -> String;

  // fixes this problem
  fn fix(&self, _base: &mut Tikibase) -> FixOutcome {
    FixOutcome::Unfixable
  }
}

pub enum FixOutcome {
  // this issue is not fixable
  Unfixable,
  // I have fixed this issue
  Fixed,
  // there was an error fixing this issue
  Error(String),
}
