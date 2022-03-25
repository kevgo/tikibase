use crate::{commands, fix, Config, Outcome, Tikibase};

pub fn fix(base: &mut Tikibase, config: &Config) -> Outcome {
    let check_result = commands::check(base, config);
    let mut fix_result = Outcome::default();
    for issue in check_result.issues {
        if let Some(fixed) = fix::fix(issue, base, config) {
            fix_result.fixes.push(fixed);
        }
    }
    fix_result
}
