use super::Fix::RemovedEmptySection;
use crate::fix;
use crate::fix::Result::Fixed;
use crate::{Location, Tikibase};

pub fn remove_section(base: &mut Tikibase, title: String, location: Location) -> fix::Result {
    let base_dir = base.dir.clone();
    let doc = base.get_doc_mut(&location.file).unwrap();
    doc.content_sections
        .retain(|section| section.title().text != title);
    doc.save(&base_dir);
    Fixed(RemovedEmptySection { title, location })
}
