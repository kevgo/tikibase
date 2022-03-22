use super::Fix;
use crate::{Location, Tikibase};

pub fn remove_occurrences_section(base: &mut Tikibase, location: Location) -> Fix {
    let base_dir = base.dir.clone();
    let doc = base.get_doc_mut(&location.file).unwrap();
    // we can simply flush the document here because
    // its "occurrences" section was filtered out when loading the document
    doc.save(&base_dir);
    Fix::RemovedObsoleteOccurrencesSection { location }
}
