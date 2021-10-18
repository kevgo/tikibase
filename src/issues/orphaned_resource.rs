use super::Problem;
use crate::fixers::Fix;
use std::fmt::{self, Display, Formatter};

/// a resource that isn't linked to
pub struct OrphanedResource {
    // This is a String and not a Path because we need a String (to print it),
    // and we already converted the Path of this orphaned resource into a String
    // during processing it.
    pub path: String,
}

impl Display for OrphanedResource {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "unused resource \"{}\"", self.path)
    }
}

impl Problem for OrphanedResource {
    fn fixer(&self) -> Option<Box<dyn Fix>> {
        None
    }
}
