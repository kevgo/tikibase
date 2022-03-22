use crate::{Issue, Position, Tikibase};
use ahash::{AHashMap, AHashSet};

pub(crate) fn scan(base: &Tikibase) -> Vec<Issue> {
    // registers variants of section titles: normalized title --> Vec<existing titles>
    let mut title_variants: AHashMap<String, Vec<FileSection>> = AHashMap::new();
    for doc in &base.docs {
        for section in &doc.content_sections {
            let section_type = section.section_type();
            title_variants
                .entry(normalize(section_type))
                .or_insert_with(Vec::new)
                .push(FileSection {
                    title: section_type.into(),
                    pos: Position {
                        file: doc.path.clone(),
                        line: section.line_number,
                    },
                });
        }
    }
    let mut issues = Vec::new();
    for (_, sections) in title_variants.drain() {
        if variants_count(&sections) < 2 {
            continue;
        }
        let mut sorted: Vec<String> = sections
            .iter()
            .map(|variant| variant.title.clone())
            .collect();
        sorted.sort();
        let mut variants = Vec::from_iter(sections);
        variants.sort(); // ensure deterministic result order in unit tests
        for variant in variants {
            issues.push(Issue::MixCapSection {
                variants: sorted.clone(),
                pos: variant.pos,
            });
        }
    }
    issues
}

#[derive(Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct FileSection {
    pos: Position,
    title: String,
}

fn variants_count(file_sections: &[FileSection]) -> usize {
    let set: AHashSet<&str> = AHashSet::from_iter(file_sections.iter().map(|fs| fs.title.as_str()));
    set.len()
}

/// normalizes the given section type
fn normalize(section_type: &str) -> String {
    section_type.to_ascii_lowercase()
}
#[cfg(test)]
mod tests {
    use crate::{test, Config, Issue, Position, Tikibase};
    use std::path::PathBuf;

    use super::FileSection;

    #[test]
    fn normalize() {
        assert_eq!(super::normalize("foo"), "foo");
        assert_eq!(super::normalize("Foo"), "foo");
        assert_eq!(super::normalize("FOO"), "foo");
    }

    #[test]
    fn different_capitalization() {
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
        let base = Tikibase::load(dir, &Config::default()).unwrap();
        let have = super::scan(&base);
        let want = vec![
            Issue::MixCapSection {
                variants: vec!["ONE".into(), "One".into(), "one".into()],
                pos: Position {
                    file: PathBuf::from("1.md"),
                    line: 2,
                },
            },
            Issue::MixCapSection {
                pos: Position {
                    file: PathBuf::from("1.md"),
                    line: 5,
                },
                variants: vec!["ONE".into(), "One".into(), "one".into()],
            },
            Issue::MixCapSection {
                pos: Position {
                    file: PathBuf::from("2.md"),
                    line: 2,
                },
                variants: vec!["ONE".into(), "One".into(), "one".into()],
            },
        ];
        pretty::assert_eq!(have, want);
    }

    #[test]
    fn same_capitalization() {
        let dir = test::tmp_dir();
        let content1 = "\
# test document

### One
content

### One
content";
        test::create_file("1.md", content1, &dir);
        let base = Tikibase::load(dir, &Config::default()).unwrap();
        let have = super::scan(&base);
        let want = vec![];
        pretty::assert_eq!(have, want);
    }

    #[test]
    fn variants_count() {
        let give: Vec<FileSection> = vec![
            FileSection {
                title: "One".into(),
                ..FileSection::default()
            },
            FileSection {
                title: "One".into(),
                ..FileSection::default()
            },
            FileSection {
                title: "one".into(),
                ..FileSection::default()
            },
        ];
        let have = super::variants_count(&give);
        let want = 2;
        assert_eq!(have, want)
    }
}
