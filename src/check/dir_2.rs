use super::scanners::orphaned_resource;
use super::{State2, doc_phase_2};
use crate::database::Directory;
use crate::filesystem;

// phase 2 `Directory` check
pub fn dir_phase_2(dir: &Directory, state: &mut State2) {
  for doc in dir.docs.values() {
    doc_phase_2(doc, &dir.config, state);
  }
  for resource in dir.resources.keys() {
    orphaned_resource::scan(&filesystem::join(&dir.relative_path, resource), state);
  }
  for dir in dir.dirs.values() {
    dir_phase_2(dir, state);
  }
}
