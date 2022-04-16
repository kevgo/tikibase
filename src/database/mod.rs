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

pub(crate) use directory::{Directory, DocumentsIterator, ResourceIterator};
pub(crate) use doc_links::DocLinks;
use document::Document;
pub(crate) use footnotes::{Footnote, Footnotes};
pub(crate) use line::Line;
pub(crate) use reference::Reference;
pub(crate) use resource::Resource;
pub(crate) use section::Section;
pub(crate) use tikibase::{LinkTargetResult, Tikibase};
