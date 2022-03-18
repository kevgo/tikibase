use crate::config;
use crate::database::Tikibase;
use crate::issue::Issue;
use crate::probes;

pub(crate) fn check(base: &Tikibase, config: &config::Data) -> Vec<Issue> {
    let mut issues = Vec::new();
    issues.extend(probes::section_duplicate::scan(base));
    issues.extend(probes::section_empty::scan(base));
    issues.extend(probes::section_capitalization::scan(base));
    issues.extend(probes::section_type::scan(base, config));
    issues.extend(probes::section_order::scan(base, config));
    issues.extend(probes::section_without_header::scan(base));
    issues.extend(probes::sources_missing::scan(base));
    let links_result = probes::link_broken::scan(base);
    issues.extend(links_result.issues);
    issues.extend(probes::image_orphaned::scan(
        base,
        &links_result.outgoing_resource_links,
    ));
    issues.extend(probes::occurrences::scan(
        base,
        &links_result.incoming_doc_links,
        &links_result.outgoing_doc_links,
    ));
    issues
}
