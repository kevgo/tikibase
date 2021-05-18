use std::path::PathBuf;

use super::{Issue, Issues};
use crate::config;
use crate::core::section::Section;
use crate::core::tikibase::Tikibase;

pub fn process(base: &Tikibase, config: &config::Data) -> Issues {
    let mut issues = Issues::new();
    let expected_order = match &config.sections {
        None => return issues,
        Some(expected_sections) => expected_sections,
    };
    for doc in &base.docs {
        if !matches_schema(doc.section_types(), expected_order) {
            issues.push(Box::new(UnorderedSections {
                file: doc.path.clone(),
            }));
        }
    }
    issues
}

/// Indicates whether the given actual contains a subset of schema, in the same order as schema.
// TODO: make actual a &[&str]
fn matches_schema(actual: Vec<&str>, schema: &[String]) -> bool {
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

/// drains the given sections vector and provides a new Vector that contains the elements ordered according to schema
fn reorder(sections: &mut Vec<Section>, schema: &[String]) -> Vec<Section> {
    let mut result: Vec<Section> = Vec::new();
    for schema_element in schema.iter() {
        let pos = sections
            .iter()
            .position(|section| section.section_type() == schema_element);
        match pos {
            None => continue,
            Some(pos) => result.push(sections.remove(pos)),
        }
    }
    result
}

/// describes the issue that a document has sections out of order
pub struct UnorderedSections {
    file: PathBuf,
}

impl Issue for UnorderedSections {
    fn describe(&self) -> String {
        format!("{}  wrong section order", self.file.to_string_lossy())
    }

    fn fix(&self, base: &mut Tikibase, config: &config::Data) -> String {
        let base_dir = base.dir.clone();
        let mut doc = base.get_doc_mut(&self.file).unwrap();
        doc.content_sections =
            reorder(&mut doc.content_sections, config.sections.as_ref().unwrap());
        doc.flush(&base_dir);
        format!("{}  fixed section order", &doc.path.to_string_lossy())
    }

    fn fixable(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {

    mod reorder {
        use super::super::reorder;
        use crate::core::section::Section;
        use crate::testhelpers::section_with_title;

        #[test]
        fn perfect_match() {
            let schema = vec!["one".to_string(), "two".to_string()];
            let mut give: Vec<Section> =
                vec![section_with_title("### one"), section_with_title("### two")];
            let have = reorder(&mut give, &schema);
            let have: Vec<&str> = have.iter().map(|section| section.section_type()).collect();
            assert_eq!(have, vec!["one", "two"]);
        }

        #[test]
        fn match_but_missing() {
            let schema = vec!["one".to_string(), "two".to_string(), "three".to_string()];
            let mut give: Vec<Section> = vec![
                section_with_title("### one"),
                section_with_title("### three"),
            ];
            let have = reorder(&mut give, &schema);
            let have: Vec<&str> = have.iter().map(|section| section.section_type()).collect();
            assert_eq!(have, vec!["one", "three"]);
        }

        #[test]
        fn wrong_order() {
            let schema = vec!["one".to_string(), "two".to_string(), "three".to_string()];
            let mut give: Vec<Section> = vec![
                section_with_title("### three"),
                section_with_title("### two"),
                section_with_title("### one"),
            ];
            let have = reorder(&mut give, &schema);
            let have: Vec<&str> = have.iter().map(|section| section.section_type()).collect();
            assert_eq!(have, vec!["one", "two", "three"]);
        }

        #[test]
        fn single_section() {
            let schema = vec!["one".to_string(), "two".to_string(), "three".to_string()];
            let mut give: Vec<Section> = vec![section_with_title("### three")];
            let have = reorder(&mut give, &schema);
            let have: Vec<&str> = have.iter().map(|section| section.section_type()).collect();
            assert_eq!(have, vec!["three"]);
        }
    }

    mod same_order {
        use super::super::matches_schema;

        #[test]
        fn perfect_match() {
            let schema = vec!["one".to_string(), "two".to_string(), "three".to_string()];
            let give = vec!["one", "two", "three"];
            assert!(matches_schema(give, &schema));
        }

        #[test]
        fn match_but_missing() {
            let schema = vec!["one".to_string(), "two".to_string(), "three".to_string()];
            let give = vec!["one", "three"];
            assert!(matches_schema(give, &schema));
        }

        #[test]
        fn mismatch() {
            let schema = vec!["one".to_string(), "two".to_string(), "three".to_string()];
            let give = vec!["two", "one"];
            assert_eq!(matches_schema(give, &schema), false);
        }

        #[test]
        fn empty() {
            let schema = vec!["one".to_string(), "two".to_string(), "three".to_string()];
            let give = vec![];
            assert_eq!(matches_schema(give, &schema), true);
        }
    }
}
