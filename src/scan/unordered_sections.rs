use crate::{Config, Document, Issue, Location};

/// populates the given issues list with all sections in this document that don't match the configured order
pub fn scan(doc: &Document, config: &Config, issues: &mut Vec<Issue>) {
    let schema_titles = match &config.sections {
        None => return,
        Some(sections) => sections,
    };
    if doc.content_sections.len() < 2 {
        // document has 0 or 1 sections --> order always matches
        return;
    }
    let mut doc_iter = doc.content_sections.iter();
    let mut doc_section_option = doc_iter.next();
    let mut schema_iter = schema_titles.iter();
    let mut schema_title_option = schema_iter.next();
    loop {
        let doc_section = match doc_section_option {
            None => return, // we reached the end of the actual list --> actual matches schema
            Some(section) => section,
        };
        let schema_title = match schema_title_option {
            None => {
                // end of schema reached but there are still unchecked sections in the document --> those are out of order
                issues.push(Issue::UnorderedSections {
                    location: Location {
                        file: doc.relative_path.clone(),
                        line: doc_section.line_number,
                        start: 0,
                        end: doc_section.title_line.text.len() as u32,
                    },
                });
                doc_section_option = doc_iter.next();
                continue;
            }
            Some(value) => value,
        };
        let doc_section_title = doc_section.human_title();
        if doc_section_title == schema_title {
            // elements match --> advance both pointers
            doc_section_option = doc_iter.next();
            schema_title_option = schema_iter.next();
            continue;
        }
        // HACK: see https://github.com/rust-lang/rust/issues/42671
        if !schema_titles.iter().any(|st| st == doc_section_title) {
            // unknown element in actual --> ignore here (there is a separate check for this)
            doc_section_option = doc_iter.next();
            continue;
        }
        // elements don't match --> advance the schema
        // (because schema might contain elements that are not in actual)
        schema_title_option = schema_iter.next();
    }
}

#[cfg(test)]
mod tests {

    use crate::{Config, Document, Issue, Location};
    use indoc::indoc;
    use std::path::PathBuf;

    #[test]
    fn mismatching() {
        let content = indoc! {"
            # Test
            ### one
            text
            ### three
            text
            ### two
            text"};
        let doc = Document::from_str("test.md", content).unwrap();
        let config = Config {
            sections: Some(vec!["one".into(), "two".into(), "three".into()]),
            ..Config::default()
        };
        let mut issues = vec![];
        super::scan(&doc, &config, &mut issues);
        let want = vec![Issue::UnorderedSections {
            location: Location {
                file: PathBuf::from("test.md"),
                line: 5,
                start: 0,
                end: 7,
            },
        }];
        assert_eq!(issues, want);
    }

    #[test]
    fn perfect_match() {
        let content = indoc! {"
            # Test
            ### one
            text
            ### two
            text
            ### three
            text"};
        let doc = Document::from_str("test.md", content).unwrap();
        let config = Config {
            sections: Some(vec!["one".into(), "two".into(), "three".into()]),
            ..Config::default()
        };
        let mut issues = vec![];
        super::scan(&doc, &config, &mut issues);
        let want = vec![];
        assert_eq!(issues, want);
    }

    #[test]
    fn match_but_missing() {
        let content = indoc! {"
            # Test
            ### one
            text
            ### two
            text
            ### three
            text"};
        let doc = Document::from_str("test.md", content).unwrap();
        let config = Config {
            sections: Some(vec!["one".into(), "three".into()]),
            ..Config::default()
        };
        let mut issues = vec![];
        super::scan(&doc, &config, &mut issues);
        let want = vec![];
        assert_eq!(issues, want);
    }

    #[test]
    fn empty() {
        let content = indoc! {"
            # Test
            ### one
            text
            ### two
            text
            ### three
            text"};
        let doc = Document::from_str("test.md", content).unwrap();
        let config = Config {
            sections: None,
            ..Config::default()
        };
        let mut issues = vec![];
        super::scan(&doc, &config, &mut issues);
        let want = vec![];
        assert_eq!(issues, want);
    }
}
