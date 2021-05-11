use crate::core::tikibase::Tikibase;
mod doc_links;
mod image_orphaned;
mod link_broken;
mod occurrences;
mod outcome;
mod section_capitalization;
mod section_duplicate;
mod section_empty;

pub fn run(base: &Tikibase) -> Vec<Box<dyn outcome::Issue>> {
    let mut issues = outcome::Issues::new();
    issues.append(section_duplicate::process(&base));
    issues.append(section_empty::process(&base));
    issues.append(section_capitalization::process(&base));
    let mut links_result = link_broken::process(&base);
    issues.append(links_result.issues);
    issues.append(&mut image_orphaned::process(
        &base,
        links_result.outgoing_resource_links,
    ));
    // let mut occ_res = occurrences::process(
    //     base,
    //     links_result.incoming_doc_links,
    //     links_result.outgoing_doc_links,
    //     fix,
    // );
    // issues.append(&mut occ_res);
    // issues.sorted()
    issues.issues()
}
