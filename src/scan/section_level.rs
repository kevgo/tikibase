use crate::{Issue, Tikibase};
use ahash::{AHashMap, AHashSet};
use std::path::Path;

pub(crate) fn scan(base: &Tikibase) -> Vec<Issue> {
    // title --> all the different levels
    let mut level_variants: AHashMap<&str, AHashMap<usize, Vec<FileSection>>> = AHashMap::new();
    for doc in &base.docs {
        for section in doc.sections() {
            let section_title = section.title();
            let section_level = section_title.level();
            level_variants
                .entry(section_title.text)
                .or_insert_with(AHashSet::new)
                .push(FileSection {
                    file: todo!(),
                    title: todo!(),
                    line: todo!(),
                    start: todo!(),
                })
        }
    }
    ()
}

#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FileSection<'a> {
    pub file: &'a Path,
    pub title: &'a str,
    pub line: u32,
    pub start: u32,
}
