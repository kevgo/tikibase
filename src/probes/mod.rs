use crate::core::tikibase::Tikibase;
mod image_orphaned;
mod link_broken;
mod occurrences;
mod outcome;
mod section_capitalization;
mod section_duplicate;
mod section_empty;

pub fn run(mut base: Tikibase, fix: bool) -> Vec<String> {
    let mut results = outcome::SortedResults::new();
    results.append(&mut section_duplicate::process(&mut base));
    results.append(&mut section_empty::process(&mut base, fix));
    results.append(&mut section_capitalization::process(&base));
    let mut links_result = link_broken::process(&base);
    results.append(&mut links_result.outcome);
    results.append(&mut image_orphaned::process(
        &base,
        links_result.resource_links,
    ));
    let (mut occ_res, _) = occurrences::process(base, links_result.doc_links, fix);
    results.append(&mut occ_res);
    results.sorted()
}
