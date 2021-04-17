use crate::core::tikibase::Tikibase;

pub fn run(base: Tikibase) -> Tikibase {
    println!("running fix in {:?}", base.dir);
    base
}
