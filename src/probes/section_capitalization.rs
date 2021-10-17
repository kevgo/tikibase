use crate::database::Tikibase;
use crate::issues;
use crate::Fix;
use ahash::{AHashMap, AHashSet};
use std::iter::FromIterator;

pub fn scan(base: &Tikibase) -> Vec<Box<dyn Fix>> {
    // registers variants of section titles: normalized title --> Vec<existing titles>
    let mut title_variants: AHashMap<String, AHashSet<String>> = AHashMap::new();
    for doc in &base.docs {
        for section in &doc.content_sections {
            let section_type = section.section_type();
            title_variants
                .entry(normalize(section_type))
                .or_insert_with(AHashSet::new)
                .insert(section_type.into());
        }
    }
    let mut issues = Vec::<Box<dyn Fix>>::new();
    for (_, variants) in title_variants.drain() {
        if variants.len() < 2 {
            continue;
        }
        let mut sorted = Vec::from_iter(variants);
        sorted.sort();
        issues.push(Box::new(issues::MixCapSection { variants: sorted }));
    }
    issues
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

    use crate::database::Tikibase;
    use crate::testhelpers::{create_file, empty_config, tmp_dir};

    #[test]
    fn progress() {
        let dir = tmp_dir();
        let content1 = "\
# test document

### ONE
content

### One
content";
        create_file("1.md", content1, &dir);
        let content2 = "\
# another document

### one
content";
        create_file("2.md", content2, &dir);
        let (base, errs) = Tikibase::load(dir, &empty_config());
        assert_eq!(errs.len(), 0);
        let have: Vec<String> = super::scan(&base)
            .iter()
            .map(|issue| issue.describe())
            .collect();
        assert_eq!(have.len(), 1);
        assert_eq!(have[0], "mixed capitalization of sections: ONE|One|one");
    }
}
