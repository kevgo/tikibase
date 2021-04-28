use crate::core::tikibase::Tikibase;
mod image_orphaned;
mod link_broken;
mod outcome;
mod section_capitalization;
mod section_duplicate;
mod section_empty;

pub fn run(base: &mut Tikibase, fix: bool) -> Vec<String> {
    let mut results = outcome::SortedResults::new();
    results.append(&mut section_duplicate::process(base));
    results.append(&mut section_empty::process(base, fix));
    results.append(&mut section_capitalization::process(base));
    let mut links_result = link_broken::process(base);
    results.append(&mut links_result.outcome);
    results.append(&mut image_orphaned::process(
        base,
        links_result.resource_links,
    ));
    results.sorted()
}
