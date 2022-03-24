//! Read/write access to the Markdown files making up the database.

mod doc_links;
pub(crate) mod document;
mod footnote;
mod line;
mod reference;
mod resource;
pub(crate) mod section;
mod tikibase;

pub(crate) use crate::database::tikibase::Tikibase;
pub(crate) use doc_links::DocLinks;
use document::Document;
pub(crate) use footnote::{Footnote, Footnotes};
pub(crate) use line::Line;
pub(crate) use reference::Reference;
use resource::Resource;
pub(crate) use section::Section;
