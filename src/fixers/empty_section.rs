use super::Fix;
use crate::Tikibase;
use std::path::PathBuf;

pub fn remove_empty_section(
    base: &mut Tikibase,
    section_type: String,
    file: PathBuf,
    line: u32,
) -> Fix {
    let base_dir = base.dir.clone();
    let doc = base.get_doc_mut(&file).unwrap();
    doc.content_sections
        .retain(|section| section.section_type() != section_type);
    doc.save(&base_dir);
    Fix::RemovedEmptySection {
        section_type,
        file,
        line,
    }
}
