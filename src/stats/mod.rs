use crate::core::tikibase;
use std::collections::HashMap;

pub fn run() {
  let tb = tikibase::in_dir(".");
  println!("documents: {}", tb.docs.len());
  println!("resources: {}", tb.resources.len());
  let section_types = collect_section_types(&tb);
  println!();
  println!("{} section types:", section_types.len());
  for (name, count) in section_types {
    println!("- {} ({})", name, count);
  }
}

fn collect_section_types(tb: &tikibase::Tikibase) -> HashMap<String, u32> {
  let mut result: HashMap<String, u32> = HashMap::new();
  for doc in &tb.docs {
    for section in &doc.content_sections {
      let section_type = section.section_type();
      match result.get(&section_type) {
        None => result.insert(section_type, 1),
        Some(count) => {
          let new = count + 1;
          result.insert(section_type, new)
        }
      };
    }
  }
  result
}
