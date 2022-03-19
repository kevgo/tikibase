mod doc_links;
pub(crate) mod document;
mod line;
mod reference;
mod resource;
pub(crate) mod section;
mod tikibase;

use crate::config;
pub(crate) use crate::database::tikibase::Tikibase;
use crate::issues::Issue;
pub(crate) use doc_links::DocLinks;
use document::Document;
pub(crate) use line::Line;
pub(crate) use reference::Reference;
use resource::Resource;
pub(crate) use section::Section;
use std::path::PathBuf;

pub fn open(path: PathBuf) -> Result<(Tikibase, config::Data), Vec<Issue>> {
    let config = config::load(&path).map_err(|issue| vec![issue])?;
    let base = Tikibase::load(path, &config)?;
    Ok((base, config))
}
