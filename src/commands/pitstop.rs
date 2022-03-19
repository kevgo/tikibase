use crate::database::Tikibase;
use crate::fixers::fix;
use crate::{commands, config};
use crate::{Fix, Issue};

pub fn pitstop(base: &mut Tikibase, config: &config::Data) -> (Vec<Issue>, Vec<Fix>) {
    let (issues, mut fixes) = commands::check(base, config);
    let mut unfixable_issues: Vec<Issue> = vec![];
    for issue in issues {
        match fix(issue.clone(), base, config) {
            Some(fix) => fixes.push(fix),
            None => unfixable_issues.push(issue),
        }
    }
    (unfixable_issues, fixes)
}
