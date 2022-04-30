use crate::check::{Issue, Location, State2};

pub fn scan(relative_path: &str, state: &mut State2) {
    if !state.linked_resources.iter().any(|l| l == relative_path) {
        state.issues.push(Issue::OrphanedResource {
            location: Location {
                file: relative_path.into(),
                line: 0,
                start: 0,
                end: 0,
            },
        });
    }
}
