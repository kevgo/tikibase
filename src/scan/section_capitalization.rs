use crate::{Issue, Location, Tikibase};
use ahash::{AHashMap, AHashSet};
use std::path::Path;

pub(crate) fn scan(base: &Tikibase) -> Vec<Issue> {
    // registers variants of section titles: normalized title --> Vec<sections with a variation of this title>
    let mut title_variants: AHashMap<String, Vec<FileSection>> = AHashMap::new();
    for doc in &base.docs {
        for section in doc.sections() {
            let (section_title, start) = section.title();
            title_variants
                .entry(normalize(section_title))
                .or_insert_with(Vec::new)
                .push(FileSection {
                    title: section_title,
                    file: &doc.path,
                    line: section.line_number,
                    start,
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
            .map(|variant| variant.title.into())
            .collect();
        variants.sort();
        for file_section in file_sections {
            issues.push(Issue::MixCapSection {
                variants: variants.clone(),
                location: Location {
                    file: file_section.file.into(),
                    line: file_section.line,
                    start: file_section.start,
                    end: file_section.start + file_section.title.len() as u32,
                },
            });
        }
    }
    issues
}

#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FileSection<'a> {
    pub file: &'a Path,
    pub title: &'a str,
    pub line: u32,
    pub start: u32,
}

impl Default for FileSection<'_> {
    fn default() -> Self {
        Self {
            file: Path::new(""),
            title: "",
            line: 0,
            start: 0,
        }
    }
}

fn variants_count(file_sections: &[FileSection]) -> usize {
    let set: AHashSet<&str> = AHashSet::from_iter(file_sections.iter().map(|fs| fs.title));
    set.len()
}

/// normalizes the given section title
fn normalize(section_title: &str) -> String {
    section_title.to_ascii_lowercase()
}
#[cfg(test)]
mod tests {
    use super::FileSection;
    use crate::{test, Config, Issue, Location, Tikibase};
    use std::path::PathBuf;

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
                location: Location {
                    file: PathBuf::from("1.md"),
                    line: 2,
                    start: 4,
                    end: 7,
                },
                variants: vec!["ONE".into(), "One".into(), "one".into()],
            },
            Issue::MixCapSection {
                location: Location {
                    file: PathBuf::from("1.md"),
                    line: 5,
                    start: 4,
                    end: 7,
                },
                variants: vec!["ONE".into(), "One".into(), "one".into()],
            },
            Issue::MixCapSection {
                location: Location {
                    file: PathBuf::from("2.md"),
                    line: 2,
                    start: 4,
                    end: 7,
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
