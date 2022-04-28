use super::scanners::{section_capitalization, section_level};
use super::Issue;
use crate::database::Document;
use ahash::AHashMap;

pub(crate) fn check_doc_2(
    doc: &Document,
    issues: &mut Vec<Issue>,
    cap_outliers: &AHashMap<String, section_capitalization::OutlierInfo>,
    level_outliers: &AHashMap<String, section_level::OutlierInfo>,
) {
    for content_section in &doc.content_sections {
        section_capitalization::phase_2(&doc.relative_path, content_section, issues, cap_outliers);
        section_level::phase_2(content_section, &doc.relative_path, issues, level_outliers);
    }
}
