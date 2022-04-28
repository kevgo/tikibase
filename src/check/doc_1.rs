use super::scanners::{
    duplicate_sections, empty_section_content, empty_section_title, footnotes, illegal_sections,
    links, section_capitalization, section_level, unordered_sections,
};
use super::State1;
use crate::database::{Directory, Document};
use crate::Config;
use std::path::Path;

// populates the given issues list with all issues in this document
pub(crate) fn check_doc_1(
    doc: &Document,
    dir: &Path,
    config: &Config,
    state_1: &mut State1,
    root: &Directory,
) {
    duplicate_sections::scan(doc, &mut state_1.issues);
    unordered_sections::scan(doc, config, &mut state_1.issues);
    footnotes::scan(doc, &mut state_1.issues);
    links::scan(
        doc,
        dir,
        &mut state_1.issues,
        &mut state_1.linked_resources,
        root,
        config,
    );
    empty_section_title::scan(&doc.title_section, &doc.relative_path, &mut state_1.issues);

    for content_section in &doc.content_sections {
        empty_section_content::scan(content_section, &doc.relative_path, &mut state_1.issues);
        empty_section_title::scan(content_section, &doc.relative_path, &mut state_1.issues);
        if config.sections.is_some() {
            illegal_sections::scan(
                content_section,
                &doc.relative_path,
                config,
                &mut state_1.issues,
            );
        } else {
            section_capitalization::phase_1(content_section, &mut state_1.title_variants);
            section_level::phase_1(content_section, &mut state_1.level_variants);
        }
    }
}
