use crate::core::tikibase::Tikibase;
mod link_broken;
mod result;
mod section_capitalization;
mod section_duplicate;
mod section_empty;

pub fn run(base: &mut Tikibase, fix: bool) -> Vec<String> {
    let mut results = result::SortedResults::new();
    results.append(&mut section_duplicate::process(base));
    results.append(&mut section_empty::process(base, fix));
    results.append(&mut section_capitalization::process(base));
    results.append(&mut link_broken::process(base));
    results.sorted()
}
