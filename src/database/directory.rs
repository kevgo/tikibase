use super::{Document, LinkTargetResult};
use crate::{Config, Issue};
use ahash::AHashMap;
use std::collections::hash_map::Iter;
use std::ffi::OsString;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::{self, Path};

/// a directory containing Tikibase files
#[derive(Default)]
pub struct Directory {
    documents: AHashMap<OsString, Document>,
    resources: Vec<OsString>,
    directories: AHashMap<OsString, Directory>,
    configuration: Config,
}

impl Directory {
    /// provides a non-consuming iterator over all documents in this directory and all its subdirectories
    pub fn documents<'a>(&'a self) -> DocumentsIterator<'a> {
        // TODO: iterate subdirs
        DocumentsIterator {
            doc_iter: self.documents.iter(),
        }
    }

    /// provides the document with the given path components if it exists in this directory or one of its subdirectories
    pub fn find_doc(
        &self,
        current_component: path::Component,
        remaining_components: path::Components,
    ) -> Option<&Document> {
        match remaining_components.next() {
            None => self.documents.get(current_component.as_os_str()),
            Some(next_component) => match self.directories.get(current_component.as_os_str()) {
                Some(dir) => dir.find_doc(next_component, remaining_components),
                None => None,
            },
        }
    }

    /// provides the document with the given path components if it exists in this directory or one of its subdirectories
    pub fn find_doc_mut(
        &mut self,
        current_component: path::Component,
        remaining_components: path::Components,
    ) -> Option<&mut Document> {
        match remaining_components.next() {
            None => self.documents.get_mut(current_component.as_os_str()),
            Some(next_component) => match self.directories.get(current_component.as_os_str()) {
                Some(dir) => dir.find_doc_mut(next_component, remaining_components),
                None => None,
            },
        }
    }

    /// indicates whether this directory or one of its subdirectories contains the given link target
    pub fn has_link_target(
        &self,
        current_component: path::Component,
        remaining_components: path::Components,
    ) -> LinkTargetResult {
        match remaining_components.next() {
            None => {
                // arrived at the filename to look for
                let (filename, target) = current_component
                    .as_os_str()
                    .to_string_lossy()
                    .split_once('#')
                    .unwrap();
                if has_extension(&filename, "md") {
                    // link points to document
                    // TODO: try to get rid of the unnecessary allocation here
                    // possibly using https://github.com/rabite0/osstrtools
                    let filename = OsString::from(filename);
                    // let os_str = OsStr::new(filename);
                    match self.documents.get(&filename) {
                        Some(doc) => match doc.has_target(target) {
                            true => return LinkTargetResult::Exists,
                            false => return LinkTargetResult::NoAnchor,
                        },
                        None => return LinkTargetResult::NoAnchor,
                    }
                } else {
                    // link points to resource
                    if target.is_empty() {
                        if self.has_resource(current_component, remaining_components) {
                            return LinkTargetResult::Exists;
                        } else {
                            return LinkTargetResult::NoFile(
                                current_component.as_os_str().to_owned(),
                            );
                        }
                    } else {
                        return LinkTargetResult::ResourceWithAnchor;
                    }
                }
            }
            Some(next_component) => match self.directories.get(current_component.as_os_str()) {
                Some(dir) => dir.has_link_target(next_component, remaining_components),
                None => LinkTargetResult::NoDir(current_component.as_os_str().to_owned()),
            },
        }
    }

    /// indicates whether this directory or one of its subdirectories contains a resource with the given path
    pub fn has_resource(
        &self,
        current_component: path::Component,
        remaining_components: path::Components,
    ) -> bool {
        match remaining_components.next() {
            None => {
                let s = current_component.as_os_str();
                self.resources.iter().any(|r| r == s)
            }
            Some(next_component) => match self.directories.get(current_component.as_os_str()) {
                Some(dir) => dir.has_resource(next_component, remaining_components),
                None => false,
            },
        }
    }

    /// provides a Directory instance containing all elements in the given directory and all its subdirectories
    pub fn load<P: AsRef<Path>>(dir: P) -> Result<Directory, Vec<Issue>> {
        let dir = dir.as_ref();
        let mut directory = Directory::default();
        let mut issues: Vec<Issue> = vec![];
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            match EntryType::from(entry) {
                EntryType::Directory => match Directory::load(dir.join(entry.file_name())) {
                    Ok(dir) => {
                        directory.directories.insert(entry.file_name(), dir);
                    }
                    Err(mut issues) => {
                        issues.append(&mut issues);
                    }
                },
                EntryType::Document => {
                    let file = File::open(entry.path()).unwrap();
                    let filename = entry.file_name();
                    match Document::from_reader(BufReader::new(file), filename) {
                        Ok(doc) => {
                            directory.documents.insert(filename, doc);
                        }
                        Err(issue) => {
                            issues.push(issue);
                        }
                    }
                }
                EntryType::Resource => {
                    directory.resources.push(entry.file_name());
                }
                EntryType::Configuration | EntryType::Ignored => {}
            }
        }
        if !issues.is_empty() {
            return Err(issues);
        }
        Ok(directory)
    }

    pub fn resources(&self) -> ResourceIterator {
        // TODO: iterate subdirectories
        ResourceIterator {
            iter: self.resources.iter(),
        }
    }
}

/// types of files that Tikibase is aware of
#[derive(Debug, PartialEq)]
pub enum EntryType {
    Directory,
    /// Markdown document
    Document,
    /// linkable resource
    Resource,
    /// Tikibase configuration file
    Configuration,
    /// ignored file
    Ignored,
}

impl From<fs::DirEntry> for EntryType {
    fn from(entry: fs::DirEntry) -> EntryType {
        let entry_type = entry.file_type().unwrap();
        if entry_type.is_file() {
            let entry_os_filename = entry.file_name();
            if entry_os_filename == "tikibase.json" {
                return EntryType::Configuration;
            }
            let entry_filestr = entry_os_filename.to_string_lossy();
            if entry_filestr.starts_with('.') {
                return EntryType::Ignored;
            }
            if has_extension(&entry_filestr, "md") {
                return EntryType::Document;
            }
            return EntryType::Resource;
        }
        if entry_type.is_dir() {
            return EntryType::Directory;
        }
        EntryType::Ignored
    }
}

/// case-insensitive comparison of file extensions
fn has_extension(path: &str, given_ext: &str) -> bool {
    let path_ext = path.rsplit('.').next().unwrap();
    path_ext.eq_ignore_ascii_case(given_ext)
}

/// iterates all documents in this directory
pub struct DocumentsIterator<'a> {
    doc_iter: Iter<'a, OsString, Document>,
}

impl<'a> Iterator for DocumentsIterator<'a> {
    type Item = &'a Document;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub struct ResourceIterator<'a> {
    iter: std::slice::Iter<'a, OsString>,
}

impl<'a> Iterator for ResourceIterator<'a> {
    type Item = &'a OsString;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    mod documents {

        #[test]
        fn foo() {
            // TODO
        }
    }

    #[test]
    fn entry_type() {
        // TODO: create test tikibase and load real DirEntry values
        // let tests = vec![
        //     ("foo.md", EntryType::Document),
        //     ("sub/foo.md", EntryType::Document),
        //     ("foo.png", EntryType::Resource),
        //     ("foo.pdf", EntryType::Resource),
        //     (".testconfig.json", EntryType::Ignored),
        // ];
        // for (give, want) in tests {
        //     let dir_entry = std::fs::DirEntry::try_from(give);
        //     let have = EntryType::from(dir_entry);
        //     assert_eq!(have, want);
        // }
    }
}
