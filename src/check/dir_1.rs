use super::{doc_phase_1, State1};
use crate::database::{paths, Directory};

// phase 1 `Directory` check
pub fn dir_phase_1(dir: &Directory, parent: &str, state: &mut State1) {
  for doc in dir.docs.values() {
    doc_phase_1(doc, dir, state);
  }
  for (dirname, dir) in &dir.dirs {
    dir_phase_1(dir, &paths::join(parent, dirname), state);
  }
}
