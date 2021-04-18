use crate::core::tikibase::Tikibase;
mod duplicate_sections;
mod empty_sections;
mod section_capitalization;

pub fn run(base: &mut Tikibase, fix: bool) -> Vec<String> {
    let mut issues = Vec::new();
    issues.append(&mut duplicate_sections::process(base));
    issues.append(&mut empty_sections::process(base, fix));
    issues.append(&mut section_capitalization::process(base));
    issues.sort();
    issues
}
