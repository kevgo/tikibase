use crate::fix::Result::{Failed, Fixed, Unfixable};
use crate::Outcome;
use crate::{commands, fix::fix, Config, Tikibase};

pub fn pitstop(base: &mut Tikibase, config: &Config) -> Outcome {
    let check_result = commands::check(base, config);
    let mut pitstop_result = Outcome::default();
    for issue in check_result.issues {
        match fix(issue.clone(), base, config) {
            Fixed(fix) => pitstop_result.fixes.push(fix),
            Failed(problem) => pitstop_result.issues.push(problem),
            Unfixable => pitstop_result.issues.push(issue),
        }
    }
    pitstop_result
}
