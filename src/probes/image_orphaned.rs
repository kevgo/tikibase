use crate::database::Tikibase;
use crate::issues;
use crate::Issue;

pub fn process(base: &Tikibase, resource_links: &[String]) -> Vec<Box<dyn Issue>> {
    let mut result = Vec::<Box<dyn Issue>>::new();
    for resource in &base.resources {
        let path = resource.path.to_string_lossy();
        if !resource_links.iter().any(|rl| rl == &path) {
            result.push(Box::new(issues::OrphanedResource { path: path.into() }));
        }
    }
    result
}
