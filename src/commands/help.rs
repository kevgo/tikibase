pub(crate) fn help() {
    println!(
        r#"Usage: tikibase <command>

Commands:
  check, c       list all issues
  fix, f         fix all auto-correctable issues
  help, h        this help screen
  pitstop, ps    fix all issues and list the unfixable ones
  stats, st      display statistics about this Tikibase
  version, v     show the currently installed version
"#
    );
}
