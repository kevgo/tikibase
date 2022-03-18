use crate::database::Tikibase;
use std::path::Path;

pub fn remove_obsolete_links(base: &mut Tikibase, file: &Path, line: u32) -> String {
    let doc = base.get_doc_mut(file).unwrap();
    // we can simply flush the document here because
    // its "occurrences" section was filtered out when loading the document
    doc.save(&base.dir);
    format!(
        "{}:{}  removed obsolete occurrences section",
        file.to_string_lossy(),
        line + 1,
    )
}
