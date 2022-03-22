use crate::{Issue, Position, Tikibase};
use ahash::{AHashMap, AHashSet};

pub(crate) fn scan(base: &Tikibase) -> Vec<Issue> {
    // registers variants of section titles: normalized title --> Vec<existing titles>
    let mut title_variants: AHashMap<String, AHashSet<FileTitle>> = AHashMap::new();
    for doc in &base.docs {
        for section in &doc.content_sections {
            let section_type = section.section_type();
            title_variants
                .entry(normalize(section_type))
                .or_insert_with(AHashSet::new)
                .insert(FileTitle {
                    title: section_type.into(),
                    pos: Position {
                        file: doc.path.clone(),
                        line: section.line_number,
                    },
                });
        }
    }
    let mut issues = Vec::new();
    for (_, variants) in title_variants.drain() {
        if variants.len() < 2 {
            continue;
        }
        let mut sorted: Vec<String> = variants
            .iter()
            .map(|variant| variant.title.clone())
            .collect();
        sorted.sort();
        let mut variants = Vec::from_iter(variants);
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

#[derive(PartialEq, Eq, Hash, Ord, PartialOrd)]
struct FileTitle {
    pos: Position,
    title: String,
}

/// normalizes the given section type
fn normalize(section_type: &str) -> String {
    section_type.to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{test, Config, Issue, Position, Tikibase};

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
}
