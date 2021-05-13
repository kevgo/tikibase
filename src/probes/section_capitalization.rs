use super::{Issue, Issues};
use crate::core::tikibase::Tikibase;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

pub fn process(base: &Tikibase) -> Issues {
    // registers variants of section titles: normalized title --> Vec<existing titles>
    // TODO: use faster hashing algorithm here
    let mut title_variants: HashMap<String, HashSet<String>> = HashMap::new();
    for doc in &base.docs {
        for section in &doc.content_sections {
            let section_type = section.section_type();
            title_variants
                .entry(normalize(&section_type))
                .or_insert_with(HashSet::new)
                .insert(section_type);
        }
    }
    let mut issues = Issues::new();
    for variants in title_variants.into_values() {
        if variants.len() < 2 {
            continue;
        }
        let mut sorted = Vec::from_iter(variants);
        sorted.sort();
        issues.push(Box::new(MixCapSection { variants: sorted }))
    }
    issues
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

    use crate::core::tikibase::Tikibase;
    use crate::testhelpers;

    #[test]
    fn progress() {
        let dir = testhelpers::tmp_dir();
        let content = "\
# test document

### ONE
content

### One
content";
        testhelpers::create_file("1.md", content, &dir);
        let content = "\
# another document

### one
content";
        testhelpers::create_file("2.md", content, &dir);
        let (mut base, errs) = Tikibase::load(dir);
        assert_eq!(errs.len(), 0);
        let have: Vec<String> = super::process(&mut base)
            .iter()
            .map(|issue| issue.describe())
            .collect();
        assert_eq!(have.len(), 1);
        assert_eq!(have[0], "mixed capitalization of sections: ONE|One|one");
    }
}
