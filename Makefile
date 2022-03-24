.DEFAULT_GOAL := help

build:  # builds the release binary
	cargo build --release --target x86_64-unknown-linux-musl

cuke:  # runs the integration tests
	rm -rf ./tmp
	cargo test --test cucumber

cukethis:  # tests only the scenario named "this"
	cargo test --test cucumber -- -t @this

fix:  # auto-corrects issues
	dprint fmt
	cargo fmt
	cargo fix

help:  # shows all available Make commands
	cat Makefile | grep '^[^ ]*:' | grep -v '.PHONY' | grep -v '.SILENT:' | grep -v help | sed 's/:.*#/#/' | column -s "#" -t

install:  # installs the binary in the system
	cargo install --path .

lint:  # checks formatting
	dprint check
	cargo clippy --all-targets --all-features -- -W clippy::pedantic -A clippy::cast_possible_wrap -A clippy::cast_possible_truncation -A clippy::missing_panics_doc -A clippy::must_use_candidate -A clippy::match_bool -A clippy::missing_errors_doc -A clippy::too-many-lines
	cargo fmt -- --check
# cargo udeps   # requires nightly
	git diff --check
	tools/actionlint

test: unit cuke lint  # runs all tests

unit:  # runs the unit tests
	cargo test

setup: setup-ci  # prepares this codebase
	cargo install cargo-edit cargo-upgrades --locked
	echo
	echo PLEASE DO THIS MANUALLY:
	echo 1. install musl, e.g. "sudo apt install musl"
	echo 2. install openssl-devel:
	echo    - Fedora: sudo dnf install openssl-devel
	echo    - Debian: sudo apt install libssl-dev pkg-config
	echo 3. cargo install cargo-edit
	echo 4. cargo install dprint

setup-ci:  # prepares the CI server
	cargo install cargo-udeps --locked
	scripts/install_actionlint

update:  # updates the dependencies
	cargo upgrade

.SILENT:
