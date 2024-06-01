use super::Issue;

pub struct EmptySectionContent {}

impl Issue for EmptySectionContent {
  fn describe(&self) -> String {
    todo!()
  }

  fn fix(&self, _base: &mut crate::database::Tikibase) -> super::FixOutcome {
    super::FixOutcome::Unfixable
  }
}
