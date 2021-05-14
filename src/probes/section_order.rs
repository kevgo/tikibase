use std::path::PathBuf;

use super::{Issue, Issues};
use crate::config;
use crate::core::section::Section;
use crate::core::tikibase::Tikibase;

pub fn process(base: &Tikibase, config: &config::Data) -> Issues {
    let mut issues = Issues::new();
    let expected_sections = match &config.allowed_sections {
        None => return issues,
        Some(expected_sections) => expected_sections,
    };
    for doc in &base.docs {
        let actual_sections: Vec<String> = doc
            .content_sections
            .iter()
            .map(|section| section.section_type())
            .collect();
        if !same_order(&actual_sections, expected_sections) {
            issues.push(Box::new(UnorderedSections {
                file: doc.path.clone(),
            }));
        }
    }
    issues
}

/// Indicates whether the given actual has the same order as the given schema.
/// Actual is allowed to be missing elements.
fn same_order(actual: &[String], schema: &[String]) -> bool {
    if actual.is_empty() {
        return true;
    }
    let mut actual_iter = actual.iter();
    let mut schema_iter = schema.iter();
    let mut actual_element = actual_iter.next();
    let mut schema_element = schema_iter.next();
    loop {
        if schema_element.is_none() && actual_element.is_none() {
            // we reached the end of both iterators --> match
            return true;
        }
        if schema_element.is_none() ^ actual_element.is_none() {
            // one of the iterators is done but the other is not --> no match
            return false;
        }
        // here there are more elements --> keep comparing them
        if actual_element == schema_element {
            // elements match --> advance both pointers
            actual_element = actual_iter.next();
            schema_element = schema_iter.next();
            continue;
        }
        // elements don't match --> advance the schema
        // (because actual might miss elements that are in schema but not the other way around)
        schema_element = schema_iter.next();
    }
}

/// provides a new Vector that contains the elements from actual ordered according to schema
fn reorder(sections: &mut Vec<Section>, schema: &[String]) -> Vec<Section> {
    let mut result: Vec<Section> = Vec::new();
    for schema_element in schema.iter() {
        let pos = sections
            .iter()
            .position(|section| &section.section_type() == schema_element);
        match pos {
            None => continue,
            Some(pos) => {
                let existing_section = sections.remove(pos);
                result.push(existing_section);
            }
        }
    }
    result
}

/// describes a document that has sections out of order
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
        doc.content_sections = reorder(
            &mut doc.content_sections,
            config.allowed_sections.as_ref().unwrap(),
        );
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
        use crate::core::line::Line;
        use crate::core::section::Section;
        use std::default::Default;

        #[test]
        fn perfect_match() {
            let schema = vec!["one".to_string(), "two".to_string()];
            let mut give: Vec<Section> = Vec::new();
            give.push(Section {
                title_line: Line {
                    text: "### one".to_string(),
                },
                ..Default::default()
            });
            give.push(Section {
                title_line: Line {
                    text: "### two".to_string(),
                },
                ..Default::default()
            });
            let have = reorder(&mut give, &schema);
            let new_order: Vec<String> =
                have.iter().map(|section| section.section_type()).collect();
            assert_eq!(new_order, vec!["one", "two"]);
        }

        fn match_but_missing() {}
        fn single_section() {}
    }

    mod same_order {
        use super::super::same_order;

        #[test]
        fn perfect_match() {
            let schema = vec!["one".to_string(), "two".to_string(), "three".to_string()];
            let give = vec!["one".to_string(), "two".to_string(), "three".to_string()];
            assert!(same_order(&give, &schema));
        }

        #[test]
        fn match_but_missing() {
            let schema = vec!["one".to_string(), "two".to_string(), "three".to_string()];
            let give = vec!["one".to_string(), "three".to_string()];
            assert!(same_order(&give, &schema));
        }

        #[test]
        fn mismatch() {
            let schema = vec!["one".to_string(), "two".to_string(), "three".to_string()];
            let give = vec!["two".to_string(), "one".to_string()];
            assert_eq!(same_order(&give, &schema), false);
        }

        #[test]
        fn empty() {
            let schema = vec!["one".to_string(), "two".to_string(), "three".to_string()];
            let give = vec![];
            assert_eq!(same_order(&give, &schema), true);
        }
    }
}
