//! Read/write access to the Markdown files making up the database.

mod directory;
pub(crate) mod document;
mod footnotes;
mod image;
mod line;
mod link;
pub(crate) mod paths;
pub(crate) mod section;
mod tikibase;

pub(crate) use crate::database::tikibase::Tikibase;
pub(crate) use directory::{Directory, EntryType};
pub(crate) use document::Document;
pub(crate) use footnotes::{Footnote, Footnotes};
pub(crate) use image::Image;
pub(crate) use line::Line;
pub(crate) use link::Link;
pub(crate) use section::Section;
