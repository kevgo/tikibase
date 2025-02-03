use super::Outcome;
use crate::Tikibase;
use gxhash::{HashMap, HashMapExt};

#[must_use]
pub fn stats(base: &Tikibase) -> Outcome {
  println!("documents: {}", base.dir.docs.len());
  println!("resources: {}", base.dir.resources.len());
  println!();
  let section_titles = collect_section_titles(base);
  println!("{} section titles:", section_titles.len());
  let mut keys: Vec<&&str> = section_titles.keys().collect();
  keys.sort();
  for key in keys {
    println!("- {} ({})", key, section_titles.get(key).unwrap());
  }
  Outcome::default()
}

fn collect_section_titles(tb: &Tikibase) -> HashMap<&str, u32> {
  let mut result: HashMap<&str, u32> = HashMap::new();
  for doc in tb.dir.docs.values() {
    for section in &doc.content_sections {
      let section_title = section.human_title();
      match result.get(section_title) {
        None => result.insert(section_title, 1),
        Some(count) => {
          let new = count + 1;
          result.insert(section_title, new)
        }
      };
    }
  }
  result
}
