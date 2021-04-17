cuke:  # runs the integration tests
	rm -rf ./tmp
	cargo test --test cucumber

test: unit cuke  # runs all tests

unit:  # runs the unit tests
	cargo clippy
	cargo test

.SILENT:
