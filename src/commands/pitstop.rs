use crate::Outcome;
use crate::{commands, config, fixers::fix, Tikibase};

pub fn pitstop(base: &mut Tikibase, config: &config::Data) -> Outcome {
    let check_outcome = commands::check(base, config);
    let mut pitstop_outcome = Outcome::default();
    for issue in check_outcome.issues {
        match fix(issue.clone(), base, config) {
            Some(fix) => pitstop_outcome.fixes.push(fix),
            None => pitstop_outcome.issues.push(issue),
        }
    }
    pitstop_outcome
}
