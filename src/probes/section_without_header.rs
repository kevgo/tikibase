use crate::database::Tikibase;
use crate::issues;
use crate::Issue;

pub fn process(base: &Tikibase) -> Vec<Box<dyn Issue>> {
    let mut issues = Vec::<Box<dyn Issue>>::new();
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
