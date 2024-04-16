all: build check test docs

fmt:
	cargo fmt

build: 
	cargo build --release

check: clippy lint

test:
	cargo test

clippy:
	cargo clippy --all-features --no-deps

lint:
	cargo fmt --check --verbose
