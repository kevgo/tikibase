use crate::core;

pub fn run() {
  let tb = core::tikibase::in_dir(".");
  println!("documents: {}", tb.docs.len());
  let c: usize = tb.docs.iter().map(|d| d.sections.len()).sum();
  // let mut sections_count = 0;
  // for doc in tb.docs {
  //   sections_count += doc.sections.len();
  // }
  println!(" sections: {}", c);
  println!("resources: {}", tb.resources.len());
}
