# Tikibase developer documentation

### Setup your development machine

1. Install openssl-devel:

   - Fedora: `sudo dnf install openssl-devel`
   - Debian: `sudo apt install libssl-dev pkg-config`

2. run `make setup`
3. run `cargo install dprint --locked`

### Run development scripts

- run `make` to see available development scripts
- run `make <script>` to execute a development script
  - e.g. `make test` or `make fix`
- run a single Cucumber scenario:
  - add a `@this` tag to the scenario
  - run `make cukethis`

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
