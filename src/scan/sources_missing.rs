use crate::{Issue, Location, Tikibase};

pub(crate) fn scan(base: &Tikibase) -> Vec<Issue> {
    let mut issues = Vec::<Issue>::new();
    for doc in &base.docs {
        let used_sources = doc.sources_used();
        let defined_source_ids = doc.sources_defined();
        for used_source in used_sources {
            if !defined_source_ids.contains(&used_source.source_ref.identifier) {
                issues.push(Issue::MissingSource {
                    location: Location {
                        file: doc.path.clone(),
                        line: used_source.line,
                        start: used_source.source_ref.start,
                        end: used_source.source_ref.end,
                    },
                    identifier: used_source.source_ref.identifier,
                });
            }
        }
    }
    issues
}
