use crate::Outcome;
use crate::{commands, config, fixers::fix, Tikibase};

pub fn pitstop(base: &mut Tikibase, config: &config::Data) -> Outcome {
    let check_result = commands::check(base, config);
    let mut pitstop_result = Outcome::default();
    for issue in check_result.issues {
        match fix(issue.clone(), base, config) {
            Some(fix) => pitstop_result.fixes.push(fix),
            None => pitstop_result.issues.push(issue),
        }
    }
    pitstop_result
}
