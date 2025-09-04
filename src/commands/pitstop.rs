use super::Outcome;
use crate::fix::Result::{Failed, Fixed, Unfixable};
use crate::fix::fix;
use crate::{Tikibase, commands};

pub fn pitstop(base: &mut Tikibase) -> Outcome {
  let check_result = commands::check(base);
  let mut pitstop_result = Outcome::default();
  for issue in check_result.issues {
    match fix(issue.clone(), base) {
      Fixed(fix) => pitstop_result.fixes.push(fix),
      Failed(problem) => pitstop_result.issues.push(problem),
      Unfixable => pitstop_result.issues.push(issue),
    }
  }
  pitstop_result
}
