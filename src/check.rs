use crate::config;
use crate::database::Tikibase;
use crate::{probes, Issues};

pub fn run(base: &Tikibase, config: &config::Data) -> Issues {
    let mut issues = Issues::new();
    issues.extend(probes::section_duplicate::process(base));
    issues.extend(probes::section_empty::process(base));
    issues.extend(probes::section_capitalization::process(base));
    issues.extend(probes::section_type::process(base, config));
    issues.extend(probes::section_order::process(base, config));
    issues.extend(probes::section_no_header::process(base));
    issues.extend(probes::sources_missing::process(base));
    let links_result = probes::link_broken::process(base);
    issues.extend(links_result.issues);
    issues.extend(probes::image_orphaned::process(
        base,
        &links_result.outgoing_resource_links,
    ));
    issues.extend(probes::occurrences::process(
        base,
        &links_result.incoming_doc_links,
        &links_result.outgoing_doc_links,
    ));
    issues
}
