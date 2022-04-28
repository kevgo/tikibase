use super::scanners::orphaned_resource;
use super::{check_doc_2, State2};
use crate::database::Directory;

// phase 2 `Directory` check
pub(crate) fn check_dir_2(dir: &Directory, state: &mut State2) {
    for doc in dir.docs.values() {
        check_doc_2(doc, &dir.config, state);
    }
    for resource in dir.resources.keys() {
        orphaned_resource::scan(dir.relative_path.join(resource), state);
    }
    for dir in dir.dirs.values() {
        check_dir_2(dir, state);
    }
}
