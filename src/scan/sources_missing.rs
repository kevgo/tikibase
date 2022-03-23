use crate::{Issue, Location, Tikibase};

pub(crate) fn scan(base: &Tikibase) -> Vec<Issue> {
    let mut issues = Vec::<Issue>::new();
    for doc in &base.docs {
        let used_sources = match doc.footnotes_used() {
            Ok(used_sources) => used_sources,
            Err(issue) => return vec![issue],
        };
        let defined_source_ids = doc.footnotes_defined();
        for used_source in used_sources {
            if !defined_source_ids.contains(&used_source.footnote_ref.identifier) {
                issues.push(Issue::MissingSource {
                    location: Location {
                        file: doc.path.clone(),
                        line: used_source.line,
                        start: used_source.footnote_ref.start,
                        end: used_source.footnote_ref.end,
                    },
                    identifier: used_source.footnote_ref.identifier,
                });
            }
        }
    }
    issues
}
