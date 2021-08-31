use crate::database::Tikibase;
use crate::issues;
use crate::Issue;

pub fn process(base: &Tikibase) -> Vec<Box<dyn Issue>> {
    let mut issues = Vec::<Box<dyn Issue>>::new();
    for doc in &base.docs {
        let used_sources = doc.sources_used();
        let defined_source_ids = doc.sources_defined();
        for used_source in used_sources {
            if !defined_source_ids.contains(&used_source.index) {
                issues.push(Box::new(issues::MissingSource {
                    file: used_source.file.to_string_lossy().into(),
                    line: used_source.line,
                    index: used_source.index,
                }));
            }
        }
    }
    issues
}
