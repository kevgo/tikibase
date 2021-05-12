use super::outcome::Issue;
use super::Tikibase;

pub fn process(base: &Tikibase, resource_links: Vec<String>) -> Vec<Box<dyn Issue>> {
    let mut result = Vec::<Box<dyn Issue>>::new();
    for resource in base.resources.iter() {
        let path = resource.path.to_string_lossy().to_string();
        if !resource_links.contains(&path) {
            result.push(Box::new(OrphanedImage { path }));
        }
    }
    result
}

pub struct OrphanedImage {
    path: String,
}

impl Issue for OrphanedImage {
    fn fix(&self, _base: &mut Tikibase) -> String {
        panic!("not fixable")
    }

    fn fixable(&self) -> bool {
        false
    }

    fn describe(&self) -> String {
        format!("unused image \"{}\"", self.path)
    }
}
