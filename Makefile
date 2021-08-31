.DEFAULT_GOAL := help

build:  # builds the release binary
	cargo build --release --target x86_64-unknown-linux-musl

cuke:  # runs the integration tests
	rm -rf ./tmp
	cargo test --test cucumber

cukethis:  # tests only the scenario named "this"
	cargo test --test cucumber -- -e this

fix:  # auto-corrects issues
	dprint fmt
	cargo fmt

help:  # shows all available Make commands
	cat Makefile | grep '^[^ ]*:' | grep -v '.PHONY' | grep -v '.SILENT:' | grep -v help | sed 's/:.*#/#/' | column -s "#" -t

install:  # installs the binary in the system
	cargo install --path .

lint:  # checks formatting
	dprint check
	cargo fmt -- --check
	cargo udeps

lint_pedantic:  # runs all lints, including false positives
	cargo clippy --all-targets --all-features -- -W clippy::pedantic -A clippy::cast_possible_wrap -A clippy::cast_possible_truncation -A clippy::missing_panics_doc -A clippy::must_use_candidate -A clippy::match_bool -A clippy::missing_errors_doc

test: unit cuke lint  # runs all tests

unit:  # runs the unit tests
	cargo clippy --all-targets --all-features -- -D warnings
	cargo test

setup:  # prepares this codebase
	cargo install cargo-udeps --locked
	echo
	echo PLEASE DO THIS MANUALLY:
	echo 1. install musl, e.g. "sudo apt install musl"
	echo 2. install openssl-devel:
	echo    - Fedora: sudo dnf install openssl-devel
	echo    - Debian: sudo apt install libssl-dev pkg-config
	echo 3. cargo install cargo-edit
	echo 4. cargo install dprint

update:  # updates the dependencies
	cargo upgrade

.SILENT:
