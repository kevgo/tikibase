const VERSION: &str = env!("CARGO_PKG_VERSION");

pub(crate) fn version() {
    println!("Tikibase v{}", VERSION);
}
