use crate::{Issue, Location, Tikibase};
use ahash::{AHashMap, AHashSet};

pub(crate) fn scan(base: &Tikibase) -> Vec<Issue> {
    // registers variants of section titles: normalized title --> Vec<sections with a variation of this title>
    let mut title_variants: AHashMap<String, Vec<FileSection>> = AHashMap::new();
    for doc in &base.docs {
        for section in doc.sections() {
            let section_type = section.section_type();
            title_variants
                .entry(normalize(section_type))
                .or_insert_with(Vec::new)
                .push(FileSection {
                    title: section_type,
                    location: Location {
                        file: doc.path.clone(),
                        line: section.line_number,
                    },
                });
        }
    }
    let mut issues = Vec::new();
    for (_, file_sections) in title_variants.drain() {
        if variants_count(&file_sections) < 2 {
            continue;
        }
        let mut variants: Vec<String> = file_sections
            .iter()
            .map(|variant| variant.title.to_string())
            .collect();
        variants.sort();
        for file_section in file_sections {
            issues.push(Issue::MixCapSection {
                variants: variants.clone(),
                location: file_section.location,
            });
        }
    }
    issues
}

#[derive(Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FileSection<'a> {
    pub location: Location,
    pub title: &'a str,
}

fn variants_count(file_sections: &[FileSection]) -> usize {
    let set: AHashSet<&str> = AHashSet::from_iter(file_sections.iter().map(|fs| fs.title));
    set.len()
}

/// normalizes the given section type
fn normalize(section_type: &str) -> String {
    section_type.to_ascii_lowercase()
}
#[cfg(test)]
mod tests {
    use crate::{test, Config, Issue, Location, Tikibase};
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
                location: Location {
                    file: PathBuf::from("1.md"),
                    line: 2,
                },
            },
            Issue::MixCapSection {
                location: Location {
                    file: PathBuf::from("1.md"),
                    line: 5,
                },
                variants: vec!["ONE".into(), "One".into(), "one".into()],
            },
            Issue::MixCapSection {
                location: Location {
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
                title: "One",
                ..FileSection::default()
            },
            FileSection {
                title: "One",
                ..FileSection::default()
            },
            FileSection {
                title: "one",
                ..FileSection::default()
            },
        ];
        let have = super::variants_count(&give);
        let want = 2;
        assert_eq!(have, want)
    }
}
