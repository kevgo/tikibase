use crate::{Issue, Tikibase};
use ahash::{AHashMap, AHashSet};
use std::iter::FromIterator;

pub(crate) fn scan(base: &Tikibase) -> Vec<Issue> {
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
    let mut issues = Vec::new();
    for (_, variants) in title_variants.drain() {
        if variants.len() < 2 {
            continue;
        }
        let mut sorted = Vec::from_iter(variants);
        sorted.sort();
        issues.push(Issue::MixCapSection { variants: sorted });
    }
    issues
}

/// normalizes the given section type
fn normalize(section_type: &str) -> String {
    section_type.to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use crate::{test, Issue, Tikibase};

    #[test]
    fn normalize() {
        assert_eq!(super::normalize("foo"), "foo");
        assert_eq!(super::normalize("Foo"), "foo");
        assert_eq!(super::normalize("FOO"), "foo");
    }

    #[test]
    fn progress() {
        let dir = test::tmp_dir();
        let content1 = "\
# test document

### ONE
content

### One
content";
        test::create_file("1.md", content1, &dir);
        let content2 = "\
# another document

### one
content";
        test::create_file("2.md", content2, &dir);
        let base = Tikibase::load(dir, &test::empty_config()).unwrap();
        let have = super::scan(&base);
        let want = vec![Issue::MixCapSection {
            variants: vec!["ONE".into(), "One".into(), "one".into()],
        }];
        pretty::assert_eq!(have, want);
    }
}