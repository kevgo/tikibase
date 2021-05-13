const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn version() {
    println!("Tikibase v{}", VERSION);
}

pub fn run() {
    println!(
        r#"Tikibase is a tool to manage knowledge bases made out of Markdown files.

Available commands:
- check, c     list all issues
- fix, f       fix all auto-correctable issues
- help, h      this help screen
- pitstop, ps  fix all issues and list the unfixable ones
- stats, st    display statistics about this Tikibase
- version, v   show the currently installed version
"#
    );
}
