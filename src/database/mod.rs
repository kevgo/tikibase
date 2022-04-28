//! Read/write access to the Markdown files making up the database.

mod directory;
pub(crate) mod document;
mod footnotes;
mod line;
mod reference;
pub(crate) mod section;
mod tikibase;

pub(crate) use crate::database::tikibase::Tikibase;
pub(crate) use directory::{Directory, EntryType};
pub(crate) use document::Document;
pub(crate) use footnotes::{Footnote, Footnotes};
pub(crate) use line::Line;
pub(crate) use reference::Reference;
pub(crate) use section::Section;
