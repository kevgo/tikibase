use super::{check_doc_2, Issue, Location, State2};
use crate::database::Directory;
use std::path::PathBuf;

// phase 2 `Directory` check
pub(crate) fn check_dir_2(dir: &Directory, state: &mut State2) {
    for doc in dir.docs.values() {
        check_doc_2(doc, &dir.config, state);
    }
    for resource in dir.resources.keys() {
        let full_path = dir.relative_path.join(resource);
        if !state.linked_resources.contains(&full_path) {
            state.issues.push(Issue::OrphanedResource {
                location: Location {
                    file: PathBuf::from(resource),
                    line: 0,
                    start: 0,
                    end: 0,
                },
            });
        }
    }
    for dir in dir.dirs.values() {
        check_dir_2(dir, state);
    }
}
