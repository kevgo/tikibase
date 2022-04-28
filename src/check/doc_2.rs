use super::scanners::{section_capitalization, section_level};
use super::{Issue, State2};
use crate::database::Document;

// phase 2 `Document` check
pub(crate) fn check_doc_2(doc: &Document, issues: &mut Vec<Issue>, state_2: &State2) {
    for content_section in &doc.content_sections {
        section_capitalization::phase_2(
            &doc.relative_path,
            content_section,
            issues,
            &state_2.capitalization_outliers,
        );
        section_level::phase_2(
            content_section,
            &doc.relative_path,
            issues,
            &state_2.level_outliers,
        );
    }
}
