use crate::{commands, config, fixers, Outcome, Tikibase};

pub fn fix(base: &mut Tikibase, config: &config::Data) -> Outcome {
    let check_outcome = commands::check(base, config);
    let mut fixes = vec![];
    for issue in check_outcome.issues {
        if let Some(fixed) = fixers::fix(issue, base, config) {
            fixes.push(fixed);
        }
    }
    Outcome {
        issues: vec![],
        fixes,
    }
}
