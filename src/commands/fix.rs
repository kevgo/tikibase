use crate::{commands, config, fixers, Fix, Issue, Tikibase};

pub fn fix(base: &mut Tikibase, config: &config::Data) -> (Vec<Issue>, Vec<Fix>) {
    let (issues, mut fixes) = commands::check(base, config);
    for issue in issues {
        if let Some(fixed) = fixers::fix(issue, base, config) {
            fixes.push(fixed);
        }
    }
    (vec![], fixes)
}
