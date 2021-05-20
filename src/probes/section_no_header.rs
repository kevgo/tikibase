use std::path::PathBuf;

use crate::core::tikibase::Tikibase;

use super::{Issue, Issues};

pub fn process(base: &Tikibase) -> Issues {
    let mut issues = Issues::new();
    for doc in &base.docs {
        for section in doc.sections() {
            if section.section_type().is_empty() {
                issues.push(Box::new(SectionNoHeader {
                    file: doc.path.clone(),
                    line: section.line_number,
                }));
            }
        }
    }
    issues
}

pub struct SectionNoHeader {
    file: PathBuf,
    line: u32,
}

impl Issue for SectionNoHeader {
    fn describe(&self) -> String {
        format!(
            "{}:{}  section has no title",
            self.file.to_string_lossy(),
            self.line + 1
        )
    }

    fn fix(&self, _base: &mut Tikibase, _config: &crate::core::config::Data) -> String {
        panic!("not fixable");
    }

    fn fixable(&self) -> bool {
        false
    }
}
