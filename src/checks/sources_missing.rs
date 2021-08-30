use super::{Issue, Issues};
use crate::database::tikibase::Tikibase;

pub fn process(base: &Tikibase) -> Issues {
    let mut issues = Issues::new();
    for doc in &base.docs {
        let used_sources = doc.sources_used();
        let defined_source_ids = doc.sources_defined();
        for used_source in used_sources {
            if !defined_source_ids.contains(&used_source.index) {
                issues.push(Box::new(MissingSource {
                    file: used_source.file.to_string_lossy().into(),
                    line: used_source.line,
                    index: used_source.index,
                }));
            }
        }
    }
    issues
}

pub struct MissingSource {
    file: String,
    line: u32,
    index: String,
}

impl Issue for MissingSource {
    fn describe(&self) -> String {
        format!(
            "{}:{}  missing source [{}]",
            self.file,
            self.line + 1,
            self.index
        )
    }

    fn fix(&self, _base: &mut Tikibase, _config: &crate::database::config::Data) -> String {
        panic!("not fixable");
    }

    fn fixable(&self) -> bool {
        false
    }
}
