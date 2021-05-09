use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

/// a link from an internal document to another internal document
pub struct DocLink {
    pub from: PathBuf,
    pub to: PathBuf,
}

pub struct DocLinks {
    links: Vec<DocLink>,
}

impl DocLinks {
    /// provides the number of Links
    pub fn len(&self) -> usize {
        self.links.len()
    }

    pub fn missing_from_file(&self, path: &Path) -> Vec<&PathBuf> {
        let mut incoming = HashSet::new();
        let mut outgoing = HashSet::new();
        for link in &self.links {
            if link.from == path {
                outgoing.insert(link.from.clone());
            }
            if link.to == path {
                incoming.insert(link.from.clone());
            }
        }
        let missing = incoming.difference(&outgoing);
        missing.into_iter().collect()
    }

    pub fn push(&mut self, link: DocLink) {
        self.links.push(link)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::probes::doc_links::DocLink;

    use super::DocLinks;

    #[test]
    fn links_from_file() {
        let doc_links = DocLinks { links: vec![] };
        doc_links.push(DocLink {
            from: PathBuf::from("1.md"),
            to: PathBuf::from("2.md"),
        });
        doc_links.push(DocLink {
            from: PathBuf::from("1.md"),
            to: PathBuf::from("3.md"),
        });
        doc_links.push(DocLink {
            from: PathBuf::from("2.md"),
            to: PathBuf::from("3.md"),
        });
        let have = doc_links.links_from_file(&PathBuf::from("1.md"));
        assert_eq!(have.len(), 2);
        assert_eq!(have[0].to_string_lossy(), "2.md");
        assert_eq!(have[1].to_string_lossy(), "3.md");
    }
}
