use crate::{Outcome, Tikibase};
use ahash::AHashMap;

pub fn stats(base: &Tikibase) -> Outcome {
    println!("documents: {}", base.docs.len());
    println!("resources: {}", base.resources.len());
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

fn collect_section_titles(tb: &Tikibase) -> AHashMap<&str, u32> {
    let mut result: AHashMap<&str, u32> = AHashMap::new();
    for doc in &tb.docs {
        for section in &doc.content_sections {
            let section_title = section.title().0;
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
