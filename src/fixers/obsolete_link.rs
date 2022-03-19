use super::Fix;
use crate::Tikibase;
use std::path::PathBuf;

pub fn remove_obsolete_links(base: &mut Tikibase, file: PathBuf, line: u32) -> Fix {
    let base_dir = base.dir.clone();
    let doc = base.get_doc_mut(&file).unwrap();
    // we can simply flush the document here because
    // its "occurrences" section was filtered out when loading the document
    doc.save(&base_dir);
    Fix::RemovedObsoleteOccurrencesSection { file, line }
}
