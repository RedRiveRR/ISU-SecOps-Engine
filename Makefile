.PHONY: build test run fmt clippy clean

# Build the project
build:
	cargo build

# Run tests
test:
	cargo test

# Format the code base
fmt:
	cargo fmt

# Run clippy for linting
clippy:
	cargo clippy --all-targets --all-features -- -D warnings

# Run the project in development mode
run:
	cargo run -- $(ARGS)

# Clean build artifacts
clean:
	cargo clean

# Standard CI check for developers
check: fmt clippy test build
