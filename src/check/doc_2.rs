use super::scanners::{obsolete_occurrences, section_capitalization, section_level};
use super::State2;
use crate::database::Document;
use crate::Config;

// phase 2 `Document` check
pub(crate) fn check_doc_2(doc: &Document, config: &Config, state: &mut State2) {
  obsolete_occurrences::scan(doc, config, &mut state.issues);
  if config.sections.is_none() {
    for content_section in &doc.content_sections {
      section_capitalization::phase_2(
        &doc.relative_path,
        content_section,
        &mut state.issues,
        &state.capitalization_outliers,
      );
      section_level::phase_2(
        content_section,
        &doc.relative_path,
        &mut state.issues,
        &state.level_outliers,
      );
    }
  }
}
