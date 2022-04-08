use crate::fix::FixResult::{Failed, Fixed, Unfixable};
use crate::{commands, fix, Config, Outcome, Tikibase};

pub fn fix(base: &mut Tikibase, config: &Config) -> Outcome {
    let check_result = commands::check(base, config);
    let mut fix_result = Outcome::default();
    for issue in check_result.issues {
        match fix::fix(issue, base, config) {
            Fixed(fix) => fix_result.fixes.push(fix),
            Failed(problem) => fix_result.issues.push(problem),
            Unfixable => {}
        }
    }
    fix_result
}
