# Tikibase developer documentation

### Setup your development machine

1. Install openssl-devel:

   - Fedora: `sudo dnf install openssl-devel`
   - Debian: `sudo apt install libssl-dev pkg-config`

2. run `make setup`

### Run development scripts

See available development scripts:

```
make
```

Execute a development script:

```
make <script>
```

Examples:

```
make test
make fix
```

Run a single Cucumber scenario:

- add a `@this` tag to the scenario
- ```
  make cukethis
  ```

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
