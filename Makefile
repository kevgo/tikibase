.DEFAULT_GOAL := help

build:  # builds the release binary
	cargo build --release --target x86_64-unknown-linux-musl

build-release:  # builds a release version of the binary
	docker run --rm --user "$(id -u)":"$(id -g)" -v "$(PWD)":/usr/src/myapp -w /usr/src/myapp rust cargo build --release
	(cd target/release && tar -czvf "../../tikibase_linux_64.tar.gz" tikibase)

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
	cat Makefile | grep '^[^ ]*:' | grep -v '.PHONY' | grep -v '.SILENT:' | grep '#' | grep -v help | sed 's/:.*#/#/' | column -s "#" -t

install:  # installs the binary in the system
	cargo install --path .

lint: lint-std-fs tools/actionlint  # checks formatting
	dprint check
	cargo clippy --all-targets --all-features -- -W clippy::pedantic -A clippy::cast_possible_wrap -A clippy::cast_possible_truncation -A clippy::missing_panics_doc -A clippy::must_use_candidate -A clippy::missing_errors_doc -A clippy::too-many-lines
	cargo fmt -- --check
# cargo udeps   # requires nightly
	git diff --check
	tools/actionlint

lint-std-fs:  # checks for occurrences of "std::fs", should use "fs_err" instead
	! grep -rn --include '*.rs' 'std::fs'

test: unit cuke lint update-json-schema  # runs all tests

unit:  # runs the unit tests
	cargo test

update-json-schema:  # updates the public JSON Schema for the config file
	cargo run -- json-schema > /dev/null
	mv tikibase.schema.json doc
	dprint fmt > /dev/null

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
# cargo install cargo-udeps --locked  # requires nightly

tools/actionlint:
	curl -s https://raw.githubusercontent.com/rhysd/actionlint/main/scripts/download-actionlint.bash | bash
	mkdir -p tools
	mv actionlint tools

update:  # updates the dependencies
	cargo upgrade

.SILENT:
