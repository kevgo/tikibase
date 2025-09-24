//! functions used in both unit and end-to-end tests

mod create_file;
mod load_file;
mod trim_end;

pub use create_file::create_file;
pub use load_file::load_file;
use trim_end::trim_end;
