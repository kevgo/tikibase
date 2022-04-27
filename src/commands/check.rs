use crate::scan::section_level;
use crate::{Outcome, Tikibase};

pub fn check(base: &mut Tikibase) -> Outcome {
    let mut issues = base.check();

    // this only makes sense if no sections are defined
    issues.extend(section_level::scan(base));

    issues.sort();
    Outcome {
        issues,
        fixes: vec![],
    }
}
