use super::outcome::Outcome;
use crate::core::tikibase::Tikibase;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

pub fn process(base: &mut Tikibase) -> Outcome {
    let mut finder = MixCapSectionFinder::new();
    for doc in &base.docs {
        finder.register(doc.title_section.section_type());
        for section in &doc.content_sections {
            finder.register(section.section_type());
        }
    }
    finder.result()
}

/// helps find sections with mixed captions
struct MixCapSectionFinder {
    /// the known section types (key=normalized version, value=actual variations)
    known_variants: HashMap<String, HashSet<String>>,
}

impl MixCapSectionFinder {
    fn new() -> MixCapSectionFinder {
        MixCapSectionFinder {
            known_variants: HashMap::new(),
        }
    }

    /// evaluates the given section type
    fn register(&mut self, section_type: String) {
        let variants = self
            .known_variants
            .entry(normalize(&section_type))
            .or_insert_with(HashSet::new);
        variants.insert(section_type);
    }

    /// provides the found sections
    fn result(self) -> Outcome {
        Outcome {
            findings: self
                .known_variants
                .into_values()
                .filter(|variants| variants.len() > 1)
                .map(|variants| {
                    let mut v_sorted = Vec::from_iter(variants);
                    v_sorted.sort();
                    format!("mixed capitalization of sections: {}", v_sorted.join("|"))
                })
                .collect(),
            fixes: Vec::new(),
        }
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

    #[test]
    fn mix_cap_section_finder() {
        let mut mcsf = super::MixCapSectionFinder::new();
        mcsf.register("same".to_string());
        mcsf.register("same".to_string());
        mcsf.register("different".to_string());
        mcsf.register("Different".to_string());
        let have = mcsf.result();
        assert_eq!(have.findings.len(), 1);
        assert_eq!(
            have.findings[0],
            "mixed capitalization of sections: Different|different",
        );
    }
}
