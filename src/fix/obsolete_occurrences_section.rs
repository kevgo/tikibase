use super::Fix;
use super::FixResult::{self, Fixed};
use crate::{Location, Tikibase};

pub fn remove_occurrences_section(base: &mut Tikibase, location: Location) -> FixResult {
    let base_dir = base.dir.clone();
    let doc = base.get_doc_mut(&location.file).unwrap();
    // we can simply flush the document here because
    // its "occurrences" section was filtered out when loading the document
    doc.save(&base_dir);
    Fixed(Fix::RemovedObsoleteOccurrencesSection { location })
}
