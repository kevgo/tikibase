use std::path::PathBuf;
use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

/// manages links to/from a document
pub struct DocLinks {
    /// key = file path, value = associated files
    pub data: HashMap<PathBuf, HashSet<PathBuf>>,
}

impl DocLinks {
    /// registers an association between doc and other_doc
    pub fn add<P: Into<PathBuf>>(&mut self, doc: P, other_doc: P) {
        let doc_path = doc.into();
        match self.data.get_mut(&doc_path) {
            None => {
                // TODO: use https://crates.io/crates/ahash as the hashing function here
                let mut docs = HashSet::new();
                docs.insert(other_doc.into());
                self.data.insert(doc_path, docs);
            }
            Some(docs) => {
                docs.insert(other_doc.into());
            }
        };
    }

    /// provides all documents that are associated with the given document
    pub fn get(&self, doc: &Path) -> HashSet<PathBuf> {
        match self.data.get(doc) {
            None => HashSet::new(),
            // TODO: return a reference here instead of cloning
            Some(result) => result.clone(),
        }
    }

    /// provides an empty DocLinks instance
    pub fn new() -> DocLinks {
        DocLinks {
            // TODO: use https://crates.io/crates/ahash as the hashing function here
            data: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::DocLinks;
    use std::path::PathBuf;

    #[test]
    fn new() {
        let doc_links = DocLinks::new();
        assert_eq!(doc_links.data.len(), 0);
    }

    #[test]
    fn add() {
        let mut doc_links = DocLinks::new();
        doc_links.add("1.md", "2.md");
        doc_links.add("1.md", "3.md");
        assert_eq!(doc_links.data.len(), 1);
        let have = doc_links.get(&PathBuf::from("1.md"));
        assert_eq!(have.len(), 2);
        assert!(have.contains(&PathBuf::from("2.md")));
        assert!(have.contains(&PathBuf::from("3.md")));
    }

    mod get {

        use crate::probes::doc_links::DocLinks;
        use std::path::PathBuf;

        #[test]
        fn exists() {
            let mut doc_links = DocLinks::new();
            doc_links.add("1.md", "2.md");
            let have = doc_links.get(&PathBuf::from("1.md"));
            assert_eq!(have.len(), 1);
            assert!(have.contains(&PathBuf::from("2.md")));
        }

        #[test]
        fn doesnt_exist() {
            let doc_links = DocLinks::new();
            let have = doc_links.get(&PathBuf::from("zonk.md"));
            assert_eq!(have.len(), 0);
        }
    }
}
