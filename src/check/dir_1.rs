use super::{check_doc_1, State1};
use crate::database::Directory;
use std::path::Path;

// phase 1 `Directory` check
pub(crate) fn check_dir_1(dir: &Directory, parent: &Path, state: &mut State1) {
    for (_filename, doc) in &dir.docs {
        check_doc_1(doc, parent, &dir.config, state);
    }
    for (dirname, dir) in &dir.dirs {
        check_dir_1(dir, &parent.join(dirname), state);
    }
}
