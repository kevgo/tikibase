use crate::scan::{
    footnotes, image_orphaned, links, occurrences, section_capitalization, section_duplicate,
    section_empty, section_level, section_order, section_title, section_without_header,
};
use crate::{Config, Outcome, Tikibase};

pub fn check(base: &mut Tikibase, config: &Config) -> Outcome {
    let mut issues = Vec::new();
    issues.extend(section_duplicate::scan(base));
    issues.extend(section_empty::scan(base));
    issues.extend(section_capitalization::scan(base));
    issues.extend(section_level::scan(base));
    issues.extend(section_title::scan(base, config));
    issues.extend(section_order::scan(base, config));
    issues.extend(section_without_header::scan(base));
    issues.extend(footnotes::scan(base));
    let links_result = links::scan(base);
    issues.extend(links_result.issues);
    issues.extend(image_orphaned::scan(
        base,
        &links_result.outgoing_resource_links,
    ));
    if let Some(bidi_links) = config.bidi_links {
        if bidi_links {
            issues.extend(occurrences::scan(
                base,
                &links_result.incoming_doc_links,
                &links_result.outgoing_doc_links,
            ));
        }
    }
    Outcome {
        issues,
        fixes: vec![],
    }
}
