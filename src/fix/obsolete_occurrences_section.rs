use super::Fix::RemovedObsoleteOccurrencesSection;
use crate::check::Location;
use crate::fix;
use crate::fix::Result::Fixed;
use crate::Tikibase;

pub fn remove_occurrences_section(base: &mut Tikibase, location: Location) -> fix::Result {
    let base_dir = base.root.clone();
    let doc = base.get_doc_mut(&location.file).unwrap();
    // we can simply flush the document here because
    // its "occurrences" section was filtered out when loading the document
    doc.save(&base_dir);
    Fixed(RemovedObsoleteOccurrencesSection { location })
}
