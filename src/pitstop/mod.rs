use crate::check;
use crate::core::tikibase::Tikibase;
use crate::fix;

pub fn run(base: Tikibase) -> Vec<String> {
    let fixed_base = fix::run(base);
    check::run(&fixed_base)
}
