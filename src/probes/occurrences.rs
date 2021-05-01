use super::outcome::Outcome;
use crate::core::tikibase::Tikibase;
use std::{collections::HashMap, path::PathBuf};

pub fn process(base: &Tikibase, doc_links: HashMap<&PathBuf, String>, fix: bool) -> Outcome {
    let result = Outcome::new();

    // determine all links to this document

    // determine all links in this document

    // determine missing links in this document

    // optionally add occurrences section

    result
}
