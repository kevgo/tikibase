RUN_THAT_APP_VERSION = 0.37.0

RTA      = tools/rta@$(RUN_THAT_APP_VERSION)
ACTIONLINT = $(RTA) actionlint
DPRINT = $(RTA) dprint

build:  # builds the release binary
	cargo build --locked --release

build-release:  # builds a release version of the binary
	docker run --rm --user "$(id -u)":"$(id -g)" -v "$(PWD)":/usr/src/myapp -w /usr/src/myapp rust cargo build --locked --release
	(cd target/release && tar -czvf "../../tikibase_linux_64.tar.gz" tikibase)

cuke: build  # runs the end-to-end tests
	rm -rf ./tmp
	cargo test --locked --test cucumber

cukethis:  # tests only the scenario named "this"
	cargo test --locked --test cucumber -- -t @this

fix: ${RTA}  # auto-corrects issues
	$(DPRINT) fmt
	cargo +nightly fmt
	cargo +nightly fix --allow-dirty
	cargo clippy --fix --allow-dirty
	cargo clippy --test=cucumber --all-features -- --deny=warnings

help:  # shows all available Make commands
	cat Makefile | grep '^[^ ]*:' | grep -v '.PHONY' | grep -v '.SILENT:' | grep '#' | grep -v help | sed 's/:.*#/#/' | column -s "#" -t

install:  # installs the binary in the system
	cargo install --locked --path .

lint: lint-std-fs ${RTA}  # checks formatting
	$(DPRINT) check
	cargo clippy --all-targets --all-features -- --deny=warnings
	cargo +nightly fmt -- --check
	git diff --check
	$(ACTIONLINT)
	cargo machete

lint-std-fs:  # checks for occurrences of "std::fs", should use "fs_err" instead
	! grep -rn --include '*.rs' 'std::fs'

test: unit cuke lint update-json-schema  # runs all tests

unit:  # runs the unit tests
	cargo test --locked

update-json-schema:  # updates the public JSON Schema for the config file
	cargo run -- json-schema > /dev/null
	mv tikibase.schema.json doc
	$(DPRINT) fmt > /dev/null

setup:  # install development dependencies on this computer
	echo
	tput bold
	echo =============================================
	echo See DEVELOPMENT.md for all installation steps
	echo =============================================
	tput sgr0
	echo
	make --no-print-dir setup-ci
	cargo install cargo-edit cargo-upgrades --locked

setup-ci:  # prepares the CI server
	rustup toolchain add nightly
	rustup component add rustfmt --toolchain nightly
	cargo install cargo-machete --locked

update: ${RTA}  # updates the dependencies
	cargo install cargo-edit
	cargo upgrade
	$(RTA) --update

# --- HELPER TARGETS --------------------------------------------------------------------------------------------------------------------------------

${RTA}:
	@rm -f tools/rta*
	@(cd tools && curl https://raw.githubusercontent.com/kevgo/run-that-app/main/download.sh | sh -s -- --version ${RUN_THAT_APP_VERSION} --name rta@${RUN_THAT_APP_VERSION})

.SILENT:
.DEFAULT_GOAL := help
