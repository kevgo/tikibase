use super::Fix;
use crate::{Position, Tikibase};

pub fn remove_empty_section(base: &mut Tikibase, section_type: String, pos: Position) -> Fix {
    let base_dir = base.dir.clone();
    let doc = base.get_doc_mut(&pos.file).unwrap();
    doc.content_sections
        .retain(|section| section.section_type() != section_type);
    doc.save(&base_dir);
    Fix::RemovedEmptySection { section_type, pos }
}
