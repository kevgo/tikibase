use crate::database::Tikibase;
use crate::issues;
use crate::Fix;

pub fn scan(base: &Tikibase, resource_links: &[String]) -> Vec<Box<dyn Fix>> {
    let mut result = Vec::<Box<dyn Fix>>::new();
    for resource in &base.resources {
        let path = resource.path.to_string_lossy();
        if !resource_links.iter().any(|rl| rl == &path) {
            result.push(Box::new(issues::OrphanedResource { path: path.into() }));
        }
    }
    result
}
