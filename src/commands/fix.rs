use crate::{commands, config, fixers, Outcome, Tikibase};

pub fn fix(base: &mut Tikibase, config: &config::Data) -> Outcome {
    let check_outcome = commands::check(base, config);
    let mut fix_outcome = Outcome::default();
    for issue in check_outcome.issues {
        if let Some(fixed) = fixers::fix(issue, base, config) {
            fix_outcome.fixes.push(fixed);
        }
    }
    fix_outcome
}
