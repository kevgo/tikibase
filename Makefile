.DEFAULT_GOAL := help

build:  # builds the release binary
	cargo build --release --target x86_64-unknown-linux-musl

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

setup:  # prepares this codebase
	echo Please do this manually:
	echo 1. install musl, e.g. "sudo apt install musl"
	echo 2. rustup target add x86_64-unknown-linux-musl --toolchain=nightly

update:  # updates the dependencies
	cargo update

.SILENT:
