use crate::database::Tikibase;
use crate::issues;
use crate::Fix;

pub fn scan(base: &Tikibase) -> Vec<Box<dyn Fix>> {
    let mut issues = Vec::<Box<dyn Fix>>::new();
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
