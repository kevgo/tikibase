use crate::core::tikibase::Tikibase;
use ahash::AHashMap;

pub fn run(base: &Tikibase) {
    println!("documents: {}", base.docs.len());
    println!("resources: {}", base.resources.len());
    let section_types = collect_section_types(&base);
    println!();
    println!("{} section types:", section_types.len());
    let mut keys: Vec<&&str> = section_types.keys().collect();
    keys.sort();
    for key in keys {
        println!("- {} ({})", key, section_types.get(key).unwrap());
    }
}

fn collect_section_types(tb: &Tikibase) -> AHashMap<&str, u32> {
    let mut result: AHashMap<&str, u32> = AHashMap::new();
    for doc in &tb.docs {
        for section in &doc.content_sections {
            let section_type = section.section_type();
            match result.get(section_type) {
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
