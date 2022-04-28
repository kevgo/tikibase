use super::{check_doc_1, State1};
use crate::database::Directory;
use std::path::Path;

// check phase 1
pub(crate) fn check_dir_1(dir: &Directory, parent: &Path, state_1: &mut State1, root: &Directory) {
    for (_filename, doc) in &dir.docs {
        check_doc_1(doc, parent, &dir.config, state_1, root);
    }
    for (dirname, dir) in &dir.dirs {
        check_dir_1(dir, &parent.join(dirname), state_1, root);
    }
}
