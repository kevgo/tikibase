# Tikibase developer documentation

- see the [Makefile](Makefile) for available development commands
- run a single Cucumber scenario: name the scenario `this`, then run
  `make cukethis`

### Architecture

- [src/main.rs](src/main.rs): the CLI binary
- [src/lib.rs](src/lib.rs): public API of the Tikibase engine
