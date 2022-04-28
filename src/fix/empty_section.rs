use super::Fix::RemovedEmptySection;
use crate::check::Location;
use crate::fix;
use crate::fix::Result::Fixed;
use crate::Tikibase;

pub fn remove_section(base: &mut Tikibase, title: String, location: Location) -> fix::Result {
    let base_dir = base.root.clone();
    let doc = base.get_doc_mut(&location.file).unwrap();
    doc.content_sections
        .retain(|section| section.human_title() != title);
    doc.save(&base_dir);
    Fixed(RemovedEmptySection { title, location })
}
