use crate::{Issue, Location, Tikibase};

pub(crate) fn scan(base: &Tikibase) -> Vec<Issue> {
    let mut issues = Vec::<Issue>::new();
    for doc in &base.docs {
        for section in doc.sections() {
            let section_title = section.title();
            if section_title.text.is_empty() {
                issues.push(Issue::SectionWithoutHeader {
                    location: Location {
                        file: doc.path.clone(),
                        line: section.line_number,
                        start: 0,
                        end: section_title.start,
                    },
                });
            }
        }
    }
    issues
}
