use crate::{commands, config, fixers::fix, Fix, Issue, Tikibase};

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
