use crate::config;
use crate::database::{Section, Tikibase};
use crate::Issue;
use std::path::PathBuf;

/// describes the issue that a document has sections out of order
pub struct UnorderedSections {
    pub file: PathBuf,
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
        doc.save(&base_dir);
        format!("{}  fixed section order", &doc.path.to_string_lossy())
    }

    fn fixable(&self) -> bool {
        true
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

#[cfg(test)]
mod tests {

    use super::reorder;
    use crate::database::Section;
    use crate::testhelpers::section_with_title;

    #[test]
    fn perfect_match() {
        let schema = vec!["one".to_string(), "two".to_string()];
        let mut give: Vec<Section> =
            vec![section_with_title("### one"), section_with_title("### two")];
        let have = reorder(&mut give, &schema);
        let have: Vec<&str> = have.iter().map(Section::section_type).collect();
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
        let have: Vec<&str> = have.iter().map(Section::section_type).collect();
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
        let have: Vec<&str> = have.iter().map(Section::section_type).collect();
        assert_eq!(have, vec!["one", "two", "three"]);
    }

    #[test]
    fn single_section() {
        let schema = vec!["one".to_string(), "two".to_string(), "three".to_string()];
        let mut give: Vec<Section> = vec![section_with_title("### three")];
        let have = reorder(&mut give, &schema);
        let have: Vec<&str> = have.iter().map(Section::section_type).collect();
        assert_eq!(have, vec!["three"]);
    }
}
