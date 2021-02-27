use super::Error;
use crate::core::tikibase::Tikibase;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

pub fn find(tb: &Tikibase) -> Vec<super::Error> {
  let mut finder = MixCapSectionFinder::new();
  for doc in &tb.docs {
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
    let normalized = normalize(&section_type);
    let variants = self
      .section_types
      .entry(normalized)
      .or_insert_with(HashSet::new);
    variants.insert(section_type);
  }

  /// provides the found sections
  fn result(self) -> Vec<Error> {
    self
      .section_types
      .into_values()
      .filter(|variants| variants.len() > 1)
      .map(|variants| {
        let mut v_sorted = Vec::from_iter(variants);
        v_sorted.sort();
        Error::MixedCapSection { variants: v_sorted }
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
    let n1 = super::normalize("foo");
    let n2 = super::normalize("Foo");
    let n3 = super::normalize("FOO");
    assert_eq!(n1, n2);
    assert_eq!(n1, n3);
    assert_eq!(n2, n3);
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
      crate::check::Error::MixedCapSection {
        variants: vec!["Different".to_string(), "different".to_string()],
      }
    );
  }
}
