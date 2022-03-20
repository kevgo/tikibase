use ahash::{AHashMap, AHashSet};
use std::path::{Path, PathBuf};

/// tracks all links between documents
#[derive(Default)]
pub struct DocLinks {
    /// key = file path, value = files that the key document points to
    pub data: AHashMap<PathBuf, AHashSet<PathBuf>>,
}

impl DocLinks {
    /// registers an association between `doc` and `other_doc`
    pub fn add<P1: Into<PathBuf>, P2: Into<PathBuf>>(&mut self, doc: P1, other_doc: P2) {
        self.data
            .entry(doc.into()) // get a mutable reference to the entry for doc
            .or_insert_with(AHashSet::new) // no entry found --> set this entry to a new HashSet
            .insert(other_doc.into()); // insert other_doc into the entry
    }

    /// provides all documents that are associated with the given document
    pub fn get<P: AsRef<Path>>(&self, doc: P) -> Option<&AHashSet<PathBuf>> {
        self.data.get(doc.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::DocLinks;
    use std::path::PathBuf;

    #[test]
    fn add() {
        let mut have = DocLinks::default();
        have.add("1.md", "2.md");
        have.add("1.md", "3.md");
        assert_eq!(have.data.len(), 1);
        let have = have.get("1.md").unwrap();
        assert_eq!(have.len(), 2);
        assert!(have.contains(&PathBuf::from("2.md")));
        assert!(have.contains(&PathBuf::from("3.md")));
    }

    mod get {
        use super::super::DocLinks;
        use std::path::PathBuf;

        #[test]
        fn exists() {
            let mut doc_links = DocLinks::default();
            doc_links.add("1.md", "2.md");
            let have = doc_links.get("1.md").unwrap();
            assert_eq!(have.len(), 1);
            assert!(have.contains(&PathBuf::from("2.md")));
        }

        #[test]
        fn doesnt_exist() {
            let doc_links = DocLinks::default();
            let have = doc_links.get("zonk.md");
            assert_eq!(have, None);
        }
    }
}
