use crate::Outcome;
use crate::{commands, config, fixers::fix, Tikibase};

pub fn pitstop(base: &mut Tikibase, config: &config::Data) -> Outcome {
    let mut outcome = commands::check(base, config);
    let mut unfixable_issues = vec![];
    for issue in outcome.issues {
        match fix(issue.clone(), base, config) {
            Some(fix) => outcome.fixes.push(fix),
            None => unfixable_issues.push(issue),
        }
    }
    Outcome {
        issues: unfixable_issues,
        fixes: outcome.fixes,
    }
}
