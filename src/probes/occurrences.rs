use super::outcome::Outcome;
use crate::core::tikibase::Tikibase;
use std::{collections::HashMap, path::PathBuf};

pub fn process(base: &Tikibase, doc_links: HashMap<&PathBuf, PathBuf>, fix: bool) -> Outcome {
    let result = Outcome::new();

    // determine all links to this document
    for doc in &base.docs {
        // determine all links in this document
        // let links_in_doc = doc_links.iter().filter(|link| link.)

        // determine missing links in this document

        // optionally add occurrences section
    }

    result
}
