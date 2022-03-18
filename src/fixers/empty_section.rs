use crate::database::Tikibase;
use std::path::Path;

pub fn remove_empty_section(
    base: &mut Tikibase,
    section_type: &str,
    filename: &Path,
    line: u32,
) -> String {
    let base_dir = base.dir.clone();
    let doc = base.get_doc_mut(filename).unwrap();
    doc.content_sections
        .retain(|section| section.section_type() != section_type);
    doc.save(&base_dir);
    format!(
        "{}:{}  removed empty section \"{}\"",
        filename.to_string_lossy(),
        line + 1,
        section_type
    )
}
