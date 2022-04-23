use crate::scan::{
    footnotes, image_orphaned, links, occurrences, section_capitalization, section_level,
    section_without_header,
};
use crate::{Outcome, Tikibase};

pub fn check(base: &mut Tikibase) -> Outcome {
    let mut issues = Vec::new();
    base.check(&mut issues);

    // This only makes sense if there are no sections defined.
    // Keep it?
    issues.extend(section_capitalization::scan(base));

    // Should we define the expected section level in tikibase.json?
    // It rarely, if ever, makes sense to allow different levels.
    issues.extend(section_level::scan(base));

    issues.extend(section_without_header::scan(base));
    issues.extend(footnotes::scan(base));
    let links_result = links::scan(base);
    issues.extend(links_result.issues);
    issues.extend(image_orphaned::scan(
        base,
        &links_result.outgoing_resource_links,
    ));
    if let Some(bidi_links) = base.dir.config.bidi_links {
        if bidi_links {
            issues.extend(occurrences::scan(
                base,
                &links_result.incoming_doc_links,
                &links_result.outgoing_doc_links,
            ));
        }
    }
    issues.sort();
    Outcome {
        issues,
        fixes: vec![],
    }
}
