use super::outcome::Outcome;
use crate::core::{document::builder_with_title_line, tikibase::Tikibase};
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

pub fn process(mut base: Tikibase, doc_links: HashMap<PathBuf, PathBuf>, fix: bool) -> Outcome {
    let mut result = Outcome::new();

    // determine all links to this document
    for doc in &mut base.docs {
        // determine all links in this document
        println!("processing doc {:?}", &doc.path);
        let outgoing: HashSet<&PathBuf> = doc_links
            .iter()
            .inspect(|link| println!("link: {:?}", link.0))
            .filter(|link| link.0 == &doc.path)
            .map(|link| link.1)
            .collect();
        println!("OUT: {:?}", &outgoing);

        // determine all links to this document
        let incoming: HashSet<&PathBuf> = doc_links
            .iter()
            .filter(|link| link.1 == &doc.path)
            .map(|link| link.0)
            .collect();
        println!("IN: {:?}", &incoming);

        // determine missing links in this document
        let missing_outgoing: HashSet<&PathBuf> = incoming.difference(&outgoing).copied().collect();
        println!("missing: {:?}", missing_outgoing);
        let mut m: Vec<&PathBuf> = missing_outgoing.iter().copied().collect();

        // no missing links --> done here
        if m.is_empty() {
            continue;
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

    result
}

#[cfg(test)]
mod tests {

    use std::{collections::HashMap, path::PathBuf};

    use crate::core::tikibase::Tikibase;
    use crate::testhelpers;

    #[test]
    fn normalize() {
        let dir = testhelpers::tmp_dir();
        let content = "\
# One

[two](two.md)
";
        testhelpers::create_file("1.md", content, &dir);
        let content = "# Two\n";
        testhelpers::create_file("2.md", content, &dir);
        let (base, errs) = Tikibase::load(dir);
        assert_eq!(errs.len(), 0);
        let mut doc_links: HashMap<PathBuf, PathBuf> = HashMap::new();
        doc_links.insert(PathBuf::from("1.md"), PathBuf::from("2.md"));
        let have = super::process(base, doc_links, false);
        assert_eq!(have.fixes.len(), 0);
        assert_eq!(have.findings, vec!["2.md  missing link to 1.md"]);
    }
}
