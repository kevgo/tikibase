use crate::core::tikibase::Tikibase;
mod empty_sections;
mod section_capitalization;

pub fn run() {
    let base = Tikibase::in_dir(".");
    let mut issues = Vec::new();
    issues.append(&mut empty_sections::find(&base));
    issues.append(&mut section_capitalization::check(&base));
    issues.sort();
    for issue in issues {
        println!("{}", issue);
    }
}
