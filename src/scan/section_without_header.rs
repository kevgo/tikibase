use crate::{Issue, Location, Tikibase};

pub(crate) fn scan(base: &Tikibase) -> Vec<Issue> {
    let mut issues = Vec::<Issue>::new();
    for doc in &base.docs {
        for section in doc.sections() {
            if section.human_title().is_empty() {
                issues.push(Issue::SectionWithoutHeader {
                    location: Location {
                        file: doc.path.clone(),
                        line: section.line_number,
                        start: 0,
                        end: section.title_text_end(),
                    },
                });
            }
        }
    }
    issues
}
