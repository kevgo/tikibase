const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn version() {
    println!("Tikibase v{}", VERSION);
    vec![]
}

pub fn run() {
    println!(
        r#"Tikibase is a tool to manage Tikibases, i.e. collections of Markdown documents in the current directory.

Available commands are:
- check (c): verify the integrity of this Tikibase
- stats (st): display statistics about this Tikibase
- version (v): show the version of the currently installed Tikibase tool
"#
    );
}
