use super::Fix;
use crate::{Position, Tikibase};

pub fn remove_occurrences_section(base: &mut Tikibase, pos: Position) -> Fix {
    let base_dir = base.dir.clone();
    let doc = base.get_doc_mut(&pos.file).unwrap();
    // we can simply flush the document here because
    // its "occurrences" section was filtered out when loading the document
    doc.save(&base_dir);
    Fix::RemovedObsoleteOccurrencesSection { pos }
}
