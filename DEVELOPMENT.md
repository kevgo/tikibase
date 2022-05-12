# Tikibase developer documentation

- see the [Makefile](Makefile) for available development commands
- run a single Cucumber scenario: add a `@this` tag to the scenario and run
  `make cukethis`

### Architecture

- [src/main.rs](src/main.rs): the CLI binary
- [src/lib.rs](src/lib.rs): public API of the Tikibase engine
