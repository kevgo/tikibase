use super::{State1, doc_phase_1};
use crate::database::Directory;
use crate::filesystem;

// phase 1 `Directory` check
pub fn dir_phase_1(dir: &Directory, parent: &str, state: &mut State1) {
  for doc in dir.docs.values() {
    doc_phase_1(doc, dir, state);
  }
  for (dirname, dir) in &dir.dirs {
    dir_phase_1(dir, &filesystem::join(parent, dirname), state);
  }
}
