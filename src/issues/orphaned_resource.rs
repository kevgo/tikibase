use crate::Issue;

/// a resource that isn't linked to
pub struct OrphanedResource {
    // This is a String and not a Path because we need a String (to print it),
    // and we already converted the Path of this orphaned resource into a String
    // during processing it.
    pub path: String,
}

impl Issue for OrphanedResource {
    fn describe(&self) -> String {
        format!("unused resource \"{}\"", self.path)
    }

    fn fixable(&self) -> bool {
        false
    }
}
