use crate::{Issue, Tikibase};

pub(crate) fn scan(base: &Tikibase) -> Vec<Issue> {
    let mut issues = Vec::<Issue>::new();
    for doc in &base.docs {
        for section in doc.sections() {
            if section.section_type().is_empty() {
                issues.push(Issue::SectionWithoutHeader {
                    file: doc.path.clone(),
                    line: section.line_number,
                });
            }
        }
    }
    issues
}