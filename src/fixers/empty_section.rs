use super::Fix;
use crate::{config, database::Tikibase, issues::EmptySection};

/// repairs the EmptySection issue
pub struct EmptySectionFixer {
    issue: EmptySection,
}

impl Fix for EmptySectionFixer {
    fn fix(self, base: &mut Tikibase, _config: &config::Data) -> String {
        let base_dir = &base.dir.clone();
        let doc = base.get_doc_mut(&self.filename).unwrap();
        doc.content_sections
            .retain(|section| section.section_type() != self.section_type);
        doc.save(base_dir.as_ref());
        format!(
            "{}:{}  removed empty section \"{}\"",
            self.filename.to_string_lossy(),
            self.line + 1,
            self.section_type
        )
    }
}
