.DEFAULT_GOAL := help

cuke:  # runs the integration tests
	rm -rf ./tmp
	cargo test --test cucumber

cukethis:  # tests only the scenario named "this"
	cargo test --test cucumber -- -e this

help:  # shows all available Make commands
	cat Makefile | grep '^[^ ]*:' | grep -v '.PHONY' | grep -v '.SILENT:' | grep -v help | sed 's/:.*#/#/' | column -s "#" -t

install:  # installs the binary in the system
	cargo install --path .

test: unit cuke  # runs all tests

unit:  # runs the unit tests
	cargo clippy
	cargo test

update:  # updates the dependencies
	cargo update

.SILENT:
