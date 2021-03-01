use crate::core::section::Section;
use crate::core::tikibase::Tikibase;

/// finds empty sections
pub fn find(base: &Tikibase) -> Vec<&Section> {
  let result = Vec::new();
  for doc in &base.docs {
    for section in &doc.content_sections {
      if section.body.len() == 0 {
        result.push(section);
      }
    }
  }
  result
}
