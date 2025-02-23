# Makefile for Cube.rs

build:
	cargo build --release
.PHONY: build

run:
	cargo run --release
.PHONY: run

fmt:
	cargo clippy -- -Dwarnings
.PHONY: fmt

clean:
	cargo clean
.PHONY: clean