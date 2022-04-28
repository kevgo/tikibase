use super::Outcome;
use crate::check::scanners::{
    duplicate_sections, empty_section_content, empty_section_title, footnotes, illegal_sections,
    links, section_capitalization, section_level, unordered_sections,
};
use crate::check::{Issue, Location};
use crate::database::{Directory, Document};
use crate::{Config, Tikibase};
use ahash::AHashMap;
use std::path::{Path, PathBuf};

pub fn check(base: &Tikibase) -> Outcome {
    let mut issues = vec![];
    let mut linked_resources = vec![];
    let mut title_variants = AHashMap::new();
    let mut level_variants = AHashMap::new();
    // round 1
    check_dir_1(
        &base.dir,
        &PathBuf::from(""),
        &mut issues,
        &mut linked_resources,
        &mut title_variants,
        &mut level_variants,
        &base.dir,
    );
    // analyze
    let title_outliers = section_capitalization::process(title_variants);
    let level_outliers = section_level::process(level_variants);
    // round 2
    check_dir_2(
        &base.dir,
        &linked_resources,
        &mut issues,
        &title_outliers,
        &level_outliers,
    );
    issues.sort();
    Outcome {
        issues,
        fixes: vec![],
    }
}

// populates the given issues list with all issues in this document
fn check_doc_1(
    doc: &Document,
    dir: &Path,
    config: &Config,
    issues: &mut Vec<Issue>,
    linked_resources: &mut Vec<PathBuf>,
    title_variants: &mut AHashMap<String, u32>,
    level_variants: &mut AHashMap<String, AHashMap<u8, u32>>,
    root: &Directory,
) {
    duplicate_sections::scan(doc, issues);
    unordered_sections::scan(doc, config, issues);
    footnotes::scan(doc, issues);
    links::scan(doc, dir, issues, linked_resources, root, config);
    empty_section_title::scan(&doc.title_section, &doc.relative_path, issues);
    for content_section in &doc.content_sections {
        empty_section_content::scan(content_section, &doc.relative_path, issues);
        empty_section_title::scan(content_section, &doc.relative_path, issues);
        if config.sections.is_some() {
            illegal_sections::scan(content_section, &doc.relative_path, config, issues);
        } else {
            section_capitalization::phase_1(content_section, title_variants);
            section_level::phase_1(content_section, level_variants);
        }
    }
}

fn check_doc_2(
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

// check phase 1
fn check_dir_1(
    dir: &Directory,
    parent: &Path,
    issues: &mut Vec<Issue>,
    linked_resources: &mut Vec<PathBuf>,
    title_variants: &mut AHashMap<String, u32>,
    level_variants: &mut AHashMap<String, AHashMap<u8, u32>>,
    root: &Directory,
) {
    for (_filename, doc) in &dir.docs {
        check_doc_1(
            doc,
            parent,
            &dir.config,
            issues,
            linked_resources,
            title_variants,
            level_variants,
            root,
        );
    }
    for (dirname, dir) in &dir.dirs {
        check_dir_1(
            dir,
            &parent.join(dirname),
            issues,
            linked_resources,
            title_variants,
            level_variants,
            root,
        );
    }
}

/// check phase 2
fn check_dir_2(
    dir: &Directory,
    linked_resources: &[PathBuf],
    issues: &mut Vec<Issue>,
    cap_outliers: &AHashMap<String, section_capitalization::OutlierInfo>,
    level_outliers: &AHashMap<String, section_level::OutlierInfo>,
) {
    for (name, doc) in &dir.docs {
        let doc_path = dir.relative_path.join(name);
        check_doc_2(doc, issues, cap_outliers, level_outliers);
        if let Some(bidi_links) = dir.config.bidi_links {
            if let Some(old_occurrences_section) = &doc.old_occurrences_section {
                if bidi_links
                    && !issues.iter().any(|issue| {
                        if let Issue::MissingLink {
                            location,
                            path: _,
                            title: _,
                        } = issue
                        {
                            location.file == doc_path
                        } else {
                            false
                        }
                    })
                {
                    issues.push(Issue::ObsoleteOccurrencesSection {
                        location: Location {
                            file: doc_path,
                            line: old_occurrences_section.line_number,
                            start: old_occurrences_section.title_text_start as u32,
                            end: old_occurrences_section.title_text_end(),
                        },
                    });
                }
            }
        }
    }
    for resource in dir.resources.keys() {
        let full_path = dir.relative_path.join(resource);
        if !linked_resources.contains(&full_path) {
            issues.push(Issue::OrphanedResource {
                location: Location {
                    file: PathBuf::from(resource),
                    line: 0,
                    start: 0,
                    end: 0,
                },
            });
        }
    }
    for dir in dir.dirs.values() {
        check_dir_2(dir, linked_resources, issues, cap_outliers, level_outliers);
    }
}

#[cfg(test)]
mod tests {
    use crate::check::{Issue, Location};
    use crate::commands::Outcome;
    use crate::{test, Tikibase};

    #[test]
    fn missing_links() {
        let dir = test::tmp_dir();
        test::create_file("1.md", "# One\n\ntext\n", &dir);
        test::create_file("2.md", "# Two\n\n[one](1.md)\n", &dir);
        test::create_file("3.md", "# Three\n\n[one](1.md)\n", &dir);
        test::create_file("tikibase.json", r#"{ "bidiLinks": true }"#, &dir);
        let base = Tikibase::load(dir).unwrap();
        let have = super::check(&base);
        let want = Outcome {
            issues: vec![
                Issue::DocumentWithoutLinks {
                    location: Location {
                        file: "1.md".into(),
                        line: 0,
                        start: 0,
                        end: 0,
                    },
                },
                Issue::MissingLink {
                    location: Location {
                        file: "1.md".into(),
                        line: 2,
                        start: 0,
                        end: 0,
                    },
                    path: "2.md".into(),
                    title: "Two".into(),
                },
                Issue::MissingLink {
                    location: Location {
                        file: "1.md".into(),
                        line: 2,
                        start: 0,
                        end: 0,
                    },
                    path: "3.md".into(),
                    title: "Three".into(),
                },
            ],
            fixes: vec![],
        };
        pretty::assert_eq!(have, want);
    }

    #[test]
    fn obsolete_occurrences() {
        let dir = test::tmp_dir();
        test::create_file("1.md", "# One\n\ntext\n### occurrences\n\n- foo", &dir);
        test::create_file("tikibase.json", r#"{ "bidiLinks": true }"#, &dir);
        let base = Tikibase::load(dir).unwrap();
        let have = super::check(&base);
        let want = Outcome {
            issues: vec![
                Issue::DocumentWithoutLinks {
                    location: Location {
                        file: "1.md".into(),
                        line: 0,
                        start: 0,
                        end: 0,
                    },
                },
                Issue::ObsoleteOccurrencesSection {
                    location: Location {
                        file: "1.md".into(),
                        line: 3,
                        start: 4,
                        end: 15,
                    },
                },
            ],
            fixes: vec![],
        };
        pretty::assert_eq!(have, want);
    }
}
