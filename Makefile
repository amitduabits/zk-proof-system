.PHONY: help test coverage bench wasm headers audit fmt clippy clean all

help:
	@echo "Available commands:"
	@echo "  make test      - Run all tests"
	@echo "  make coverage  - Run tests with coverage"
	@echo "  make bench     - Run benchmarks"
	@echo "  make wasm      - Build WASM artifacts"
	@echo "  make headers   - Generate C headers"
	@echo "  make audit     - Run security audit"
	@echo "  make fmt       - Format code"
	@echo "  make clippy    - Run linter"
	@echo "  make clean     - Clean build artifacts"
	@echo "  make all       - Run all checks"

test:
	cargo test --all-features

coverage:
	powershell -ExecutionPolicy Bypass -File scripts/run-tests-coverage.ps1

bench:
	powershell -ExecutionPolicy Bypass -File scripts/run-benchmarks.ps1

wasm:
	powershell -ExecutionPolicy Bypass -File scripts/build-wasm.ps1

headers:
	powershell -ExecutionPolicy Bypass -File scripts/generate-c-headers.ps1

audit:
	cargo audit

fmt:
	cargo fmt --all

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

clean:
	cargo clean
	powershell -Command "Remove-Item -Recurse -Force -ErrorAction SilentlyContinue wasm-artifacts, c-headers, benchmark-results"

all: fmt clippy test audit
