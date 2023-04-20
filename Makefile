all: fmt clippy test build
fmt:
	cargo fmt --all -- --check
clippy:
	cargo clippy -- -D warnings
test:
	cargo test
build:
	cargo build