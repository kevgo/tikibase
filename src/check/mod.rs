use crate::core::tikibase::Tikibase;
mod empty_sections;
mod section_capitalization;

pub fn process(base: &mut Tikibase, fix: bool) -> Vec<String> {
    let mut issues = Vec::new();
    issues.append(&mut empty_sections::process(base, fix));
    issues.append(&mut section_capitalization::process(base));
    issues.sort();
    issues
}
