.POSIX:
.SILENT:
.PHONY: \
	all \
	audit \
	build \
	cargo-check \
	clean \
	clean-cargo \
	clippy \
	doc \
	install \
	lint \
	publish \
	rustfmt \
	test
.IGNORE: \
	clean \
	clean-cargo

all: build

audit:
	cargo audit

build:
	cargo build --release

cargo-check:
	cargo check

clean: \
	clean-cargo

clean-cargo:
	cargo clean

clippy:
	cargo clippy

doc:
	cargo doc

lint: \
	cargo-check \
	clippy \
	doc \
	rustfmt

publish:
	cargo publish

rustfmt:
	cargo fmt

test:
	cargo test
