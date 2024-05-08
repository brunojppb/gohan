all: test

build:
	@cargo build --all-features

doc:
	@RUSTC_BOOTSTRAP=1 RUSTDOCFLAGS="--cfg=docsrs" cargo doc --no-deps --all-features

test: cargotest

cargotest:
	@echo "CARGO TESTS"
	@rustup component add rustfmt 2> /dev/null
	@CI=0 cargo test

format:
	@rustup component add rustfmt 2> /dev/null
	@cargo fmt --all

format-check:
	@rustup component add rustfmt 2> /dev/null
	@cargo fmt --all -- --check

lint:
	@rustup component add clippy 2> /dev/null
	@cargo clippy --all-targets --workspace -- --deny warnings

.PHONY: all doc test cargotest format format-check lint
