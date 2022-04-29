use super::{check_doc_1, State1};
use crate::database::Directory;

// phase 1 `Directory` check
pub(crate) fn check_dir_1(dir: &Directory, parent: &str, state: &mut State1) {
    for doc in dir.docs.values() {
        check_doc_1(doc, &dir.relative_path, &dir.config, state);
    }
    for (dirname, dir) in &dir.dirs {
        check_dir_1(dir, &format!("{}/{}", parent, dirname), state);
    }
}
