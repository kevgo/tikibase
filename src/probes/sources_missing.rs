use super::Issues;
use crate::core::tikibase::Tikibase;

pub fn process(base: &Tikibase) -> Issues {
    let mut issues = Issues::new();
    for doc in &base.docs {
        let used_sources = doc.sources_used();
        let sources_section = match doc.get_section("links") {
            Some(section) => section,
            None => return issues,
        };
    }
    issues
}
