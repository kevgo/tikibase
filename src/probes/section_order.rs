use crate::config;
use crate::database::Tikibase;
use crate::issue::Issue;

pub(crate) fn scan(base: &Tikibase, config: &config::Data) -> Vec<Issue> {
    let mut issues = Vec::new();
    let expected_order = match &config.sections {
        None => return issues,
        Some(expected_sections) => expected_sections,
    };
    for doc in &base.docs {
        if !matches_schema(&doc.section_types(), expected_order) {
            issues.push(Issue::UnorderedSections {
                file: doc.path.clone(),
            });
        }
    }
    issues
}

/// Indicates whether the given actual contains a subset of schema, in the same order as schema.
fn matches_schema(actual: &[&str], schema: &[String]) -> bool {
    if actual.len() < 2 {
        // 0 or 1 elements --> order always matches
        return true;
    }
    let mut actual_iter = actual.iter();
    let mut actual_element = actual_iter.next();
    let mut schema_iter = schema.iter();
    let mut schema_element = schema_iter.next();
    loop {
        let actual_value = match actual_element {
            None => return true, // we reached the end of the actual list --> actual matches schema
            Some(value) => value,
        };
        let schema_value = match schema_element {
            None => return false, // we reached the end of schema but there are still elements in actual --> no match
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

    mod matches_schema {
        use super::super::matches_schema;

        #[test]
        fn perfect_match() {
            let schema = vec!["one".to_string(), "two".to_string(), "three".to_string()];
            let give = vec!["one", "two", "three"];
            assert!(matches_schema(&give, &schema));
        }

        #[test]
        fn match_but_missing() {
            let schema = vec!["one".to_string(), "two".to_string(), "three".to_string()];
            let give = vec!["one", "three"];
            assert!(matches_schema(&give, &schema));
        }

        #[test]
        fn mismatch() {
            let schema = vec!["one".to_string(), "two".to_string(), "three".to_string()];
            let give = vec!["two", "one"];
            assert!(!matches_schema(&give, &schema));
        }

        #[test]
        fn empty() {
            let schema = vec!["one".to_string(), "two".to_string(), "three".to_string()];
            let give = Vec::new();
            assert!(matches_schema(&give, &schema));
        }
    }
}
