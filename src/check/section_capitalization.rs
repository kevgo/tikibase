use crate::core::tikibase::Tikibase;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

/// describes a set of sections with mixed capitalization
#[derive(Debug, PartialEq)]
pub struct MixedCapSection {
  pub variants: Vec<String>,
}

pub fn find(base: &Tikibase) -> Vec<MixedCapSection> {
  let mut finder = MixCapSectionFinder::new();
  for doc in &base.docs {
    finder.register(doc.title_section.section_type());
    for section in &doc.content_sections {
      finder.register(section.section_type());
    }
  }
  finder.result()
}

struct MixCapSectionFinder {
  /// the known section types (key=normalized version, value=existing variations)
  section_types: HashMap<String, HashSet<String>>,
}

impl MixCapSectionFinder {
  fn new() -> MixCapSectionFinder {
    MixCapSectionFinder {
      section_types: HashMap::new(),
    }
  }

  /// evaluates the given section type
  fn register(&mut self, section_type: String) {
    let variants = self
      .section_types
      .entry(normalize(&section_type))
      .or_insert_with(HashSet::new);
    variants.insert(section_type);
  }

  /// provides the found sections
  fn result(self) -> Vec<MixedCapSection> {
    self
      .section_types
      .into_values()
      .filter(|variants| variants.len() > 1)
      .map(|variants| {
        let mut v_sorted = Vec::from_iter(variants);
        v_sorted.sort();
        MixedCapSection { variants: v_sorted }
      })
      .collect()
  }
}

/// normalizes the given section type
fn normalize(section_type: &str) -> String {
  section_type.to_ascii_lowercase()
}

#[cfg(test)]
mod tests {

  #[test]
  fn normalize() {
    assert_eq!(super::normalize("foo"), "foo");
    assert_eq!(super::normalize("Foo"), "foo");
    assert_eq!(super::normalize("FOO"), "foo");
  }

  #[test]
  fn mix_cap_section_finder() {
    let mut mcsf = super::MixCapSectionFinder::new();
    mcsf.register("same".to_string());
    mcsf.register("same".to_string());
    mcsf.register("different".to_string());
    mcsf.register("Different".to_string());
    let have = mcsf.result();
    assert_eq!(have.len(), 1);
    assert_eq!(
      have[0],
      super::MixedCapSection {
        variants: vec!["Different".to_string(), "different".to_string()],
      }
    );
  }
}
