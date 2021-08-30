use crate::database::Tikibase;
use crate::{issues, Issues};

pub fn process(base: &Tikibase) -> Issues {
    let mut issues = Issues::new();
    for doc in &base.docs {
        for section in doc.sections() {
            if section.section_type().is_empty() {
                issues.push(Box::new(issues::SectionWithoutHeader {
                    file: doc.path.clone(),
                    line: section.line_number,
                }));
            }
        }
    }
    issues
}
