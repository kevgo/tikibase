use crate::{Issue, Location, Tikibase};
use ahash::AHashMap;

/// finds all duplicate sections in the given Tikibase
pub(crate) fn scan(base: &Tikibase) -> Vec<Issue> {
    let mut issues = Vec::new();
    for doc in &base.docs {
        // section title -> [lines with this section]
        let mut sections_lines: AHashMap<&str, Vec<(u32, u32)>> = AHashMap::new();
        for section in doc.sections() {
            let (section_title, start) = section.title();
            sections_lines
                .entry(section_title)
                .or_insert_with(Vec::new)
                .push((section.line_number, start))
        }
        for (title, lines) in sections_lines.drain() {
            if lines.len() > 1 {
                for (line, start) in lines {
                    issues.push(Issue::DuplicateSection {
                        location: Location {
                            file: doc.path.clone(),
                            line,
                            start,
                            end: start + title.len() as u32,
                        },
                        title: title.into(),
                    });
                }
            }
        }
    }
    issues
}

#[cfg(test)]
mod tests {
    use super::scan;
    use crate::{test, Config, Issue, Location, Tikibase};
    use std::path::PathBuf;

    #[test]
    fn duplicate_sections() {
        let dir = test::tmp_dir();
        let content = "\
# test document

### One
content
### One
content";
        test::create_file("test.md", content, &dir);
        let base = Tikibase::load(dir, &Config::default()).unwrap();
        let have = scan(&base);
        let want = vec![
            Issue::DuplicateSection {
                location: Location {
                    file: PathBuf::from("test.md"),
                    line: 2,
                    start: 4,
                    end: 14,
                },
                title: "One".into(),
            },
            Issue::DuplicateSection {
                location: Location {
                    file: PathBuf::from("test.md"),
                    line: 4,
                    start: 4,
                    end: 14,
                },
                title: "One".into(),
            },
        ];
        pretty::assert_eq!(have, want)
    }
}
