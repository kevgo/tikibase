use crate::{Issue, Location, Tikibase};

pub(crate) fn scan(base: &Tikibase) -> Vec<Issue> {
    let mut issues = Vec::<Issue>::new();
    for doc in &base.docs {
        for section in doc.sections() {
            if section.title().is_empty() {
                issues.push(Issue::SectionWithoutHeader {
                    location: Location {
                        file: doc.path.clone(),
                        line: section.line_number,
                    },
                });
            }
        }
    }
    issues
}
