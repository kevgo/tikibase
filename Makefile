# dev tooling and versions
RUN_THAT_APP_VERSION = 0.5.0

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

fix: tools/run-that-app@${RUN_THAT_APP_VERSION}  # auto-corrects issues
	tools/rta dprint fmt
	cargo fmt
	cargo fix
	cargo clippy --fix

help:  # shows all available Make commands
	cat Makefile | grep '^[^ ]*:' | grep -v '.PHONY' | grep -v '.SILENT:' | grep '#' | grep -v help | sed 's/:.*#/#/' | column -s "#" -t

install:  # installs the binary in the system
	cargo install --locked --path .

lint: lint-std-fs tools/run-that-app@${RUN_THAT_APP_VERSION}  # checks formatting
	tools/rta dprint check
	cargo clippy --all-targets --all-features -- --deny=warnings
	cargo fmt -- --check
# cargo udeps   # requires nightly
	git diff --check
	tools/rta actionlint

lint-std-fs:  # checks for occurrences of "std::fs", should use "fs_err" instead
	! grep -rn --include '*.rs' 'std::fs'

test: unit cuke lint update-json-schema  # runs all tests

unit:  # runs the unit tests
	cargo test

update-json-schema:  # updates the public JSON Schema for the config file
	cargo run -- json-schema > /dev/null
	mv tikibase.schema.json doc
	tools/rta dprint fmt > /dev/null

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

update: tools/run-that-app@${RUN_THAT_APP_VERSION}  # updates the dependencies
	cargo upgrade
	tools/rta --update

# --- HELPER TARGETS --------------------------------------------------------------------------------------------------------------------------------

tools/run-that-app@${RUN_THAT_APP_VERSION}:
	@rm -f tools/run-that-app* tools/rta
	@(cd tools && curl https://raw.githubusercontent.com/kevgo/run-that-app/main/download.sh | sh)
	@mv tools/run-that-app tools/run-that-app@${RUN_THAT_APP_VERSION}
	@ln -s run-that-app@${RUN_THAT_APP_VERSION} tools/rta

.SILENT:
.DEFAULT_GOAL := help
