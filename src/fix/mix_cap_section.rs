use super::inconsistent_levels::title_at_level;
use super::Fix::NormalizedSectionCapitalization;
use crate::database::Tikibase;
use crate::fix::Result::Fixed;
use crate::{fix, Location};

pub fn normalize_capitalization(
    base: &mut Tikibase,
    location: Location,
    section_level: u8,
    old_capitalization: String,
    new_capitalization: String,
) -> fix::Result {
    let base_dir = base.dir.clone();
    let doc = base.find_doc_mut(&location.file).unwrap();
    let section = doc.section_with_title_mut(&old_capitalization).unwrap();
    section.title_line.text = title_at_level(&new_capitalization, section_level as usize);
    doc.save(&base_dir);
    Fixed(NormalizedSectionCapitalization {
        location,
        old_capitalization,
        new_capitalization,
    })
}
