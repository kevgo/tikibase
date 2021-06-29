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

lint:  # checks formatting
	cargo fmt -- --check

test: unit cuke lint  # runs all tests

unit:  # runs the unit tests
	cargo clippy --all-targets --all-features -- -D warnings
	cargo test

setup:  # prepares this codebase
	echo Please do this manually:
	echo 1. install musl, e.g. "sudo apt install musl"
	echo 2. rustup target add x86_64-unknown-linux-musl --toolchain=nightly
	echo 3. install openssl-devel:
	echo    - Fedora: sudo dnf install openssl-devel
	echo    - Debian: sudo apt install libssl-dev
	echo 4. cargo install cargo-edit

update:  # updates the dependencies
	cargo upgrade

.SILENT:
