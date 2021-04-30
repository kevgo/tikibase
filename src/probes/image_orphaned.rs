use super::Tikibase;
use crate::core::error::{Outcome, Outcomes};

pub fn process(base: &Tikibase, resource_links: Vec<String>) -> Outcomes {
    let mut result = Outcomes::new();
    for resource in base.resources.iter() {
        let path = &resource.path.to_string_lossy().to_string();
        if !resource_links.contains(&path) {
            result.push(Outcome::UserError(format!("unused image \"{}\"", path)));
        }
    }
    result
}
