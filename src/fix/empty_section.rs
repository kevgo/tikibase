use super::Fix;
use crate::{Location, Tikibase};

pub fn remove_empty_section(base: &mut Tikibase, title: String, location: Location) -> Fix {
    let base_dir = base.dir.clone();
    let doc = base.get_doc_mut(&location.file).unwrap();
    doc.content_sections
        .retain(|section| section.title().text != title);
    doc.save(&base_dir);
    Fix::RemovedEmptySection { title, location }
}
