use super::Fix;
use crate::config;
use crate::database::Tikibase;
use crate::issues::ObsoleteLink;

pub struct ObsoleteLinkFixer<'a> {
    pub issue: &'a ObsoleteLink,
}

impl Fix for ObsoleteLinkFixer<'_> {
    fn fix(&self, base: &mut Tikibase, _config: &config::Data) -> String {
        let base_dir = base.dir.clone();
        let doc = base.get_doc_mut(&self.issue.file).unwrap();
        // we can simply flush the document here because
        // its "occurrences" section was filtered out when loading the document
        doc.save(&base_dir);
        format!(
            "{}:{}  removed obsolete occurrences section",
            self.issue.file.to_string_lossy(),
            self.issue.line + 1,
        )
    }
}
