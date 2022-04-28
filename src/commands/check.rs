use crate::{Outcome, Tikibase};

pub fn check(base: &mut Tikibase) -> Outcome {
    Outcome {
        issues: base.check(),
        fixes: vec![],
    }
}
