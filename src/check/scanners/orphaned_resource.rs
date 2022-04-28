use crate::check::{Issue, Location, State2};
use std::path::PathBuf;

pub fn scan(relative_path: PathBuf, state: &mut State2) {
    if !state.linked_resources.contains(&relative_path) {
        state.issues.push(Issue::OrphanedResource {
            location: Location {
                file: relative_path,
                line: 0,
                start: 0,
                end: 0,
            },
        });
    }
}
