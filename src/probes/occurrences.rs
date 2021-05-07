use super::outcome::Outcome;
use crate::core::{document::builder_with_title_line, tikibase::Tikibase};
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

struct MissingOccurrence {
    path: PathBuf,
    title: String,
}

pub fn process(mut base: Tikibase, doc_links: HashMap<PathBuf, PathBuf>, fix: bool) -> Outcome {
    let mut result = Outcome::new();

    let mut missing_occurrences = HashMap::<PathBuf, Vec<MissingOccurrence>>::new();

    for doc in &base.docs {
        // determine outgoing links
        println!("processing doc {:?}", &doc.path);
        let outgoing: HashSet<&PathBuf> = doc_links
            .iter()
            .inspect(|link| println!("link: {:?}", link.0))
            .filter(|link| link.0 == &doc.path)
            .map(|link| link.1)
            .collect();
        println!("OUT: {:?}", &outgoing);

        // determine incoming links
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

        // register missing occurrences
        m.sort();
        missing_occurrences.insert(
            doc.path.clone(),
            missing_outgoing
                .into_iter()
                .map(|path| base.get_doc(path).unwrap())
                .map(|doc| MissingOccurrence {
                    path: doc.path,
                    title: doc.title(),
                })
                .collect(),
        );
    }

    if fix {
        for missing_occurrence in missing_occurrences {
            let doc = base.get_doc_mut(&missing_occurrence.0).unwrap();
            let mut section_builder =
                builder_with_title_line("### occurrences".to_string(), doc.last_line() + 1);
            for missing in missing_occurrence.1 {
                let missing_doc = base.get_doc_mut(missing).unwrap();
                section_builder.add_body_line(format!(
                    "- [{}]({})",
                    missing_doc.title(),
                    &missing.to_string_lossy()
                ));
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
        }
    } else {
        for missing_occurrence in missing_occurrences {
            for missing_file in missing_occurrence.1 {
                result.findings.push(format!(
                    "{}  missing link to {}",
                    missing_occurrence.0.to_string_lossy(),
                    missing_file.to_string_lossy()
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
    fn process_false() {
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

    #[test]
    fn process_true() {
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
        let have = super::process(base, doc_links, true);
        assert_eq!(have.fixes, vec!["2.md:1  added occurrences section"]);
        assert_eq!(have.findings.len(), 0);
    }
}
