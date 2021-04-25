cuke:  # runs the integration tests
	rm -rf ./tmp
	cargo test --test cucumber

cukethis:  # tests only the scenario named "this"
	cargo test --test cucumber -- -e this

install:  # installs the binary in the system
	cargo install --path .

test: unit cuke  # runs all tests

unit:  # runs the unit tests
	cargo clippy
	cargo test

.SILENT:
