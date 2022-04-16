//! Read/write access to the Markdown files making up the database.

mod directory;
mod doc_links;
pub(crate) mod document;
mod footnotes;
mod line;
mod reference;
mod resource;
pub(crate) mod section;
mod tikibase;

use std::str::Split;

pub(crate) use directory::{Directory, DocumentsIterator, ResourceIterator};
pub(crate) use doc_links::DocLinks;
use document::Document;
pub(crate) use footnotes::{Footnote, Footnotes};
pub(crate) use line::Line;
pub(crate) use reference::Reference;
pub(crate) use resource::Resource;
pub(crate) use section::Section;
pub(crate) use tikibase::{LinkTargetResult, Tikibase};

/// iterates the segments within a relative link between documents,
/// like foo/bar.md#target
pub struct LinkSegmentIterator<'a> {
    link: &'a str,
    slash_iter: Split<'a, char>,
    pos: usize,
}

impl<'a> LinkSegmentIterator<'a> {
    pub fn new(link: &'a str) -> LinkSegmentIterator<'a> {
        let slash_iter = link.split('/');
        LinkSegmentIterator {
            link,
            slash_iter,
            pos: 0,
        }
    }
}

impl<'a> Iterator for LinkSegmentIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self.slash_iter.next() {
            Some(element) => Some(element),
            None => todo!(),
        }
    }
}
