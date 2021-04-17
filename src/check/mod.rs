use crate::core::tikibase::Tikibase;
mod empty_sections;
mod section_capitalization;

pub fn run(base: &Tikibase) -> Vec<String> {
    let mut issues = Vec::new();
    issues.append(&mut empty_sections::find(&base));
    issues.append(&mut section_capitalization::check(&base));
    issues.sort();
    issues
}
