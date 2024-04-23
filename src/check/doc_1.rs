use super::scanners::{
  duplicate_sections, empty_section_content, empty_section_title, footnotes, illegal_sections,
  links, section_capitalization, section_level, unordered_sections,
};
use super::State1;
use crate::database::{Directory, Document};

// phase 1 `Document` check
pub fn check_doc_1(doc: &Document, dir: &Directory, state: &mut State1) {
  duplicate_sections::scan(doc, &mut state.issues);
  unordered_sections::scan(doc, &dir.config, &mut state.issues);
  footnotes::scan(doc, &mut state.issues);
  links::scan(
    doc,
    dir,
    &mut state.issues,
    &mut state.linked_resources,
    state.base_dir,
  );
  empty_section_title::scan(&doc.title_section, &doc.relative_path, &mut state.issues);
  for content_section in &doc.content_sections {
    empty_section_content::scan(content_section, &doc.relative_path, &mut state.issues);
    empty_section_title::scan(content_section, &doc.relative_path, &mut state.issues);
    if dir.config.sections.is_some() {
      illegal_sections::scan(
        content_section,
        &doc.relative_path,
        &dir.config,
        &mut state.issues,
      );
    } else {
      section_capitalization::phase_1(content_section, &mut state.capitalization_variants);
      section_level::phase_1(content_section, &mut state.level_variants);
    }
  }
}
