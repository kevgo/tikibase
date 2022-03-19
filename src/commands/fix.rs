use crate::commands;
use crate::database::Tikibase;
use crate::fixers;
use crate::{config, Fix, Issue};

pub fn fix(base: &mut Tikibase, config: &config::Data) -> (Vec<Issue>, Vec<Fix>) {
    let (issues, mut fixes) = commands::check(base, config);
    for issue in issues {
        if let Some(fixed) = fixers::fix(issue, base, config) {
            fixes.push(fixed);
        }
    }
    (vec![], fixes)
}
