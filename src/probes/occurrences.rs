use super::outcome::Outcome;
use crate::core::{document::builder_with_title_line, tikibase::Tikibase};
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

pub fn process(
    mut base: Tikibase,
    doc_links: HashMap<PathBuf, PathBuf>,
    fix: bool,
) -> (Outcome, Tikibase) {
    let mut result = Outcome::new();

    // determine all links to this document
    for doc in &mut base.docs {
        // determine all links in this document
        let outgoing: HashSet<&PathBuf> = doc_links
            .iter()
            .filter(|link| link.0 == &doc.path)
            .map(|link| link.1)
            .collect();

        // determine all links to this document
        let incoming: HashSet<&PathBuf> = doc_links
            .iter()
            .filter(|link| link.1 == &doc.path)
            .map(|link| link.0)
            .collect();

        // determine missing links in this document
        let missing_outgoing: HashSet<&PathBuf> =
            outgoing.intersection(&incoming).map(|path| *path).collect();
        let mut m: Vec<&PathBuf> = missing_outgoing.iter().map(|path| *path).collect();

        // no missing links --> done here
        if m.len() == 0 {
            return (result, base);
        }

        m.sort();

        // optionally add occurrences section
        if fix {
            let mut section_builder =
                builder_with_title_line("### occurrences".to_string(), doc.last_line() + 1);
            for missing in missing_outgoing {
                section_builder.add_body_line(format!("- {}", missing.to_string_lossy()));
            }
            let occurrences_section = section_builder.result().unwrap();
            let line = occurrences_section.line_number;
            doc.content_sections.push(occurrences_section);
            doc.flush(&base.dir);
            result.fixes.push(format!(
                "{}:{}  added occurrences section",
                doc.path.to_string_lossy(),
                line
            ));
        } else {
            for missing in missing_outgoing {
                result.findings.push(format!(
                    "{}  missing link to {}",
                    doc.path.to_string_lossy(),
                    missing.to_string_lossy()
                ));
            }
        }
    }

    (result, base)
}
