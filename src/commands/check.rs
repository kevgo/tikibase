use crate::scan::{section_capitalization, section_level};
use crate::{Outcome, Tikibase};

pub fn check(base: &mut Tikibase) -> Outcome {
    let mut issues = vec![];
    let mut linked_resources = vec![];
    base.check(&mut issues, &mut linked_resources);

    // This only makes sense if there are no sections defined.
    // Keep it?
    issues.extend(section_capitalization::scan(base));

    // Should we define the expected section level in tikibase.json?
    // It rarely, if ever, makes sense to allow different levels.
    // This might still make sense if tikibase.json defines no sections.
    issues.extend(section_level::scan(base));

    issues.sort();
    Outcome {
        issues,
        fixes: vec![],
    }
}
