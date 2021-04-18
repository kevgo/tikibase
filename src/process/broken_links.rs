use super::link_targets;

pub fn process(base: &mut Tikibase) -> Result {
    let existing_targets = link_targets::find(&base);
    for doc in &base.docs {
        let targets_used_by_doc = doc.used_targets();
    }
}
