use super::outcome::Issue;
use crate::core::tikibase::Tikibase;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

pub fn process(base: &Tikibase) -> Vec<Box<dyn Issue>> {
    // registers variants of section titles: normalized title --> Vec<existing titles>
    // TODO: use faster hashing algorithm here
    let mut title_variants: HashMap<String, HashSet<String>> = HashMap::new();
    for doc in &base.docs {
        for section in doc.sections() {
            let section_type = section.section_type();
            title_variants
                .entry(normalize(&section_type))
                .or_insert_with(HashSet::new)
                .insert(section_type);
        }
    }
    let mut result = Vec::<Box<dyn Issue>>::new();
    for variants in title_variants.into_values() {
        if variants.len() < 2 {
            continue;
        }
        let mut sorted = Vec::from_iter(variants);
        sorted.sort();
        result.push(Box::new(MixCapSection { variants: sorted }))
    }
    result
}

/// describes the issue that sections have mixed capitalization
pub struct MixCapSection {
    variants: Vec<String>,
}

impl Issue for MixCapSection {
    fn describe(&self) -> String {
        format!(
            "mixed capitalization of sections: {}",
            self.variants.join("|")
        )
    }

    fn fix(&self, _base: &mut Tikibase) -> String {
        panic!("not fixable")
    }

    fn fixable(&self) -> bool {
        false
    }
}

/// normalizes the given section type
fn normalize(section_type: &str) -> String {
    section_type.to_ascii_lowercase()
}

#[cfg(test)]
mod tests {

    #[test]
    fn normalize() {
        assert_eq!(super::normalize("foo"), "foo");
        assert_eq!(super::normalize("Foo"), "foo");
        assert_eq!(super::normalize("FOO"), "foo");
    }

    // TODO: add test for process
}
