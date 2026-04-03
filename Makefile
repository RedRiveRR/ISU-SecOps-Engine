.PHONY: build test run fmt lint audit ci clean help

# Default target
_default: help

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
lint:
	cargo clippy --all-targets -- -D warnings

# Run security audit
audit:
	cargo deny check

# Run full local CI check
ci: fmt lint audit test build

# Clean build artifacts
clean:
	cargo clean

# Show help
help:
	@echo "Aviable commands:"
	@echo "  make build    - Build the project"
	@echo "  make test     - Run tests"
	@echo "  make fmt      - Format the code"
	@echo "  make lint     - Run clippy (strict)"
	@echo "  make audit    - Run security audit"
	@echo "  make ci       - Run all checks (fmt, lint, audit, test, build)"
	@echo "  make clean    - Clean target directory"
