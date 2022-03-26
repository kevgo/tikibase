use crate::{Config, Issue, Tikibase};

pub(crate) fn scan(base: &Tikibase, config: &Config) -> Vec<Issue> {
    let mut issues = Vec::new();
    let expected_order = match &config.sections {
        None => return issues,
        Some(expected_sections) => expected_sections,
    };
    for doc in &base.docs {
        if let Some(mismatching) = first_mismatching(&doc.section_titles(), expected_order) {
            let section = doc.section_with_title(&mismatching).unwrap();
            issues.push(Issue::UnorderedSections {
                location: crate::Location {
                    file: doc.path.clone(),
                    line: section.line_number,
                    start: 0,
                    end: section.title_line.text.len() as u32,
                },
            });
        }
    }
    issues
}

/// provides the first element of actual that doesn't match schema.
fn first_mismatching(actual: &[&str], schema: &[String]) -> Option<String> {
    if actual.len() < 2 {
        // 0 or 1 elements --> order always matches
        return None;
    }
    let mut actual_iter = actual.iter();
    let mut actual_element = actual_iter.next();
    let mut schema_iter = schema.iter();
    let mut schema_element = schema_iter.next();
    loop {
        let actual_value = match actual_element {
            None => return None, // we reached the end of the actual list --> actual matches schema
            Some(value) => value,
        };
        let schema_value = match schema_element {
            None => return Some((*actual_value).to_string()), // we reached the end of schema but there are still elements in actual --> no match
            Some(value) => value,
        };

        if actual_value == schema_value {
            // elements match --> advance both pointers
            actual_element = actual_iter.next();
            schema_element = schema_iter.next();
            continue;
        }

        // HACK: see https://github.com/rust-lang/rust/issues/42671
        if !schema.iter().any(|s| s == actual_value) {
            // unknown element in actual --> ignore here (there is a separate check for this)
            actual_element = actual_iter.next();
            continue;
        }

        // elements don't match --> advance the schema
        // (because schema might contain elements that are not in actual)
        schema_element = schema_iter.next();
    }
}

#[cfg(test)]
mod tests {
    use crate::database::Tikibase;
    use crate::{test, Config, Issue, Location};
    use indoc::indoc;
    use std::path::PathBuf;

    mod matches_schema {
        use super::super::first_mismatching;

        #[test]
        fn perfect_match() {
            let schema = vec!["one".to_string(), "two".to_string(), "three".to_string()];
            let give = vec!["one", "two", "three"];
            assert_eq!(first_mismatching(&give, &schema), None);
        }

        #[test]
        fn match_but_missing() {
            let schema = vec!["one".to_string(), "two".to_string(), "three".to_string()];
            let give = vec!["one", "three"];
            assert_eq!(first_mismatching(&give, &schema), None);
        }

        #[test]
        fn mismatch() {
            let schema = vec!["one".to_string(), "two".to_string(), "three".to_string()];
            let give = vec!["one", "three", "two"];
            assert_eq!(first_mismatching(&give, &schema), Some("two".into()));
        }

        #[test]
        fn empty() {
            let schema = vec!["one".to_string(), "two".to_string(), "three".to_string()];
            let give = Vec::new();
            assert_eq!(first_mismatching(&give, &schema), None);
        }
    }

    #[test]
    fn mismatching_order() {
        let dir = test::tmp_dir();
        let content1 = indoc! {"
            # Test
            ### one
            text
            ### three
            text
            ### two
            text"};
        test::create_file("1.md", content1, &dir);
        let content2 = indoc! {"
            # another
            [1](1.md) content"};
        test::create_file("2.md", content2, &dir);
        let config = Config {
            sections: Some(vec!["one".into(), "two".into(), "three".into()]),
            ignore: None,
        };
        let base = Tikibase::load(dir, &Config::default()).unwrap();
        let have = super::scan(&base, &config);
        let want = vec![Issue::UnorderedSections {
            location: Location {
                file: PathBuf::from("1.md"),
                line: 5,
                start: 0,
                end: 7,
            },
        }];
        assert_eq!(have, want);
    }
}
