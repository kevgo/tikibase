use crate::core::tikibase;

pub fn run() {
  let tb = tikibase::in_dir(".");
  println!("documents: {}", tb.docs.len());
  println!(
    " sections: {}",
    tb.docs.iter().map(|d| d.sections.len()).sum::<usize>()
  );
  println!("resources: {}", tb.resources.len());
}
