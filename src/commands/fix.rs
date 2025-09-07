use super::Outcome;
use crate::fix::Result::{Failed, Fixed, Unfixable};
use crate::{Tikibase, commands, fix};

pub fn fix(base: &mut Tikibase) -> Outcome {
  let check_result = commands::check(base);
  let mut fix_result = Outcome::default();
  for issue in check_result.issues {
    match fix::fix(issue, base) {
      Fixed(fix) => fix_result.fixes.push(fix),
      Failed(problem) => fix_result.issues.push(problem),
      Unfixable => {}
    }
  }
  fix_result
}
