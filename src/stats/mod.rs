use crate::core;

pub fn run() {
  let tb = core::tikibase::in_dir(".");
  println!("documents: {}", tb.docs.len());
}
