# Tikibase developer documentation

- see the [Makefile](Makefile) for available development commands
- run a single Cucumber scenario: add a `@this` tag to the scenario and run
  `make cukethis`

### Debug the executable

- open file [.vscode/launch.json](.vscode/launch.json)
- check the entry `Debug executable 'tikibase'`
  - `args`
  - `cwd`
- switch VSCode to the `Debug` view
- set a breakpoint
- choose `Debug executable 'tikibase'`

### Architecture

- [src/main.rs](src/main.rs): the CLI binary
- [src/lib.rs](src/lib.rs): public API of the Tikibase engine
