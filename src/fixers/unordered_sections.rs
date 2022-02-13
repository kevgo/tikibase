use crate::{
    config,
    database::{Section, Tikibase},
    issues::UnorderedSections,
};

use super::Fix;

pub struct UnorderedSectionFixer<'a> {
    pub issue: &'a UnorderedSections,
}

impl Fix for UnorderedSectionFixer<'_> {
    fn fix(&self, base: &mut Tikibase, config: &config::Data) -> String {
        let base_dir = base.dir.clone();
        let mut doc = base.get_doc_mut(&self.issue.file).unwrap();
        doc.content_sections =
            reorder(&mut doc.content_sections, config.sections.as_ref().unwrap());
        doc.save(&base_dir);
        format!("{}  fixed section order", &doc.path.to_string_lossy())
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

    #[test]
    fn perfect_match() {
        let schema = vec!["one".to_string(), "two".to_string()];
        let mut give: Vec<Section> = vec![
            Section::with_title("### one"),
            Section::with_title("### two"),
        ];
        let have = reorder(&mut give, &schema);
        let have: Vec<&str> = have.iter().map(Section::section_type).collect();
        assert_eq!(have, vec!["one", "two"]);
    }

    #[test]
    fn match_but_missing() {
        let schema = vec!["one".to_string(), "two".to_string(), "three".to_string()];
        let mut give: Vec<Section> = vec![
            Section::with_title("### one"),
            Section::with_title("### three"),
        ];
        let have = reorder(&mut give, &schema);
        let have: Vec<&str> = have.iter().map(Section::section_type).collect();
        assert_eq!(have, vec!["one", "three"]);
    }

    #[test]
    fn wrong_order() {
        let schema = vec!["one".to_string(), "two".to_string(), "three".to_string()];
        let mut give: Vec<Section> = vec![
            Section::with_title("### three"),
            Section::with_title("### two"),
            Section::with_title("### one"),
        ];
        let have = reorder(&mut give, &schema);
        let have: Vec<&str> = have.iter().map(Section::section_type).collect();
        assert_eq!(have, vec!["one", "two", "three"]);
    }

    #[test]
    fn single_section() {
        let schema = vec!["one".to_string(), "two".to_string(), "three".to_string()];
        let mut give: Vec<Section> = vec![Section::with_title("### three")];
        let have = reorder(&mut give, &schema);
        let have: Vec<&str> = have.iter().map(Section::section_type).collect();
        assert_eq!(have, vec!["three"]);
    }
}