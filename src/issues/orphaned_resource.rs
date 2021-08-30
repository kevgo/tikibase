use crate::config;
use crate::database::Tikibase;
use crate::Issue;

/// a resource that isn't linked to
pub struct OrphanedResource {
    // TODO: make Path?
    pub path: String,
}

impl Issue for OrphanedResource {
    fn fix(&self, _base: &mut Tikibase, _config: &config::Data) -> String {
        panic!("not fixable")
    }

    fn fixable(&self) -> bool {
        false
    }

    fn describe(&self) -> String {
        format!("unused resource \"{}\"", self.path)
    }
}
