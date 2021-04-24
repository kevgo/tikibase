use crate::core::tikibase::Tikibase;
mod broken_links;
mod duplicate_sections;
mod empty_sections;
mod result;
mod section_capitalization;

pub fn run(base: &mut Tikibase, fix: bool) -> Vec<String> {
    let mut results = result::SortedResults::new();
    results.append(&mut duplicate_sections::process(base));
    results.append(&mut empty_sections::process(base, fix));
    results.append(&mut section_capitalization::process(base));
    results.sorted()
}
