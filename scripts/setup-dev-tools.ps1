# Save this as scripts/setup-dev-tools.ps1

Write-Host "=== ZK Proof System Development Tools Setup ===" -ForegroundColor Cyan
Write-Host "This will install all required development tools" -ForegroundColor Yellow

# Function to check if a command exists
function Test-CommandExists {
    param($Command)
    $null -ne (Get-Command $Command -ErrorAction SilentlyContinue)
}

# Create scripts directory
New-Item -ItemType Directory -Force -Path "scripts" | Out-Null

# 1. RUST TOOLCHAIN
Write-Host "`n[1/4] Setting up Rust toolchain..." -ForegroundColor Green

# Install nightly toolchain
Write-Host "Installing Rust nightly..." -ForegroundColor Yellow
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly

# Install cargo tools
$cargoTools = @(
    @{name="cargo-audit"; purpose="Security scanning"},
    @{name="cargo-criterion"; purpose="Benchmarking"},
    @{name="cargo-tarpaulin"; purpose="Code coverage"},
    @{name="wasm-pack"; purpose="WASM compilation"},
    @{name="cbindgen"; purpose="C bindings generation"},
    @{name="mdbook"; purpose="Documentation"},
    @{name="mdbook-katex"; purpose="Math rendering"}
)

foreach ($tool in $cargoTools) {
    $toolName = $tool.name
    Write-Host "Installing $toolName ($($tool.purpose))..." -ForegroundColor Yellow
    
    if (Test-CommandExists $toolName.Replace("cargo-", "")) {
        Write-Host "  $toolName already installed" -ForegroundColor DarkGray
    } else {
        cargo install $toolName
    }
}

# 2. VERIFICATION TOOLS
Write-Host "`n[2/4] Setting up verification tools..." -ForegroundColor Green

# Z3 check
Write-Host "Checking Z3 installation..." -ForegroundColor Yellow
try {
    python -c "import z3; print('Z3 version:', z3.get_version_string())"
    Write-Host "  Z3 is installed" -ForegroundColor Green
} catch {
    Write-Host "  Z3 not found. Install with: pip install z3-solver" -ForegroundColor Yellow
}

# Note about Kani
Write-Host "`nNote: Kani model checker is not supported on Windows" -ForegroundColor Yellow

# 3. DOCUMENTATION TOOLS
Write-Host "`n[3/4] Documentation tools setup..." -ForegroundColor Green

# Check for LaTeX
Write-Host "Checking for LaTeX installation..." -ForegroundColor Yellow
if (Test-CommandExists "pdflatex") {
    Write-Host "  LaTeX already installed" -ForegroundColor DarkGray
} else {
    Write-Host "  LaTeX not found. Please install MiKTeX from:" -ForegroundColor Yellow
    Write-Host "  https://miktex.org/download" -ForegroundColor White
}

# 4. VERIFY INSTALLATIONS
Write-Host "`n[4/4] Verifying installations..." -ForegroundColor Green

$tools = @(
    "rustc", "cargo", "cargo-audit", "cargo-criterion", 
    "cargo-tarpaulin", "wasm-pack", "cbindgen", "mdbook"
)

$allInstalled = $true
foreach ($tool in $tools) {
    if (Test-CommandExists $tool) {
        $version = & $tool --version 2>&1 | Select-Object -First 1
        Write-Host "OK $tool : $version" -ForegroundColor Green
    } else {
        Write-Host "X $tool : Not installed" -ForegroundColor Red
        $allInstalled = $false
    }
}

# Create Makefile for cross-platform support
Write-Host "`nCreating Makefile..." -ForegroundColor Yellow
$makefile = @'
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
'@

# Go up one directory to write Makefile in project root
Push-Location ..
$makefile | Out-File -Encoding UTF8 "Makefile"
Pop-Location

# Final summary
Write-Host "`n=== Setup Summary ===" -ForegroundColor Cyan

if ($allInstalled) {
    Write-Host "All core tools installed successfully!" -ForegroundColor Green
} else {
    Write-Host "Some tools are missing. Please install them manually." -ForegroundColor Yellow
}

Write-Host "`nNext steps:" -ForegroundColor Yellow
Write-Host "1. Run 'cargo test' to verify project builds" -ForegroundColor White
Write-Host "2. Run individual scripts in the scripts/ folder" -ForegroundColor White
Write-Host "3. Use 'make help' to see available commands (requires Make)" -ForegroundColor White

Write-Host "`nAvailable automation scripts:" -ForegroundColor Green
Write-Host "  .\scripts\run-tests-coverage.ps1  - Generate coverage report" -ForegroundColor Gray
Write-Host "  .\scripts\build-wasm.ps1          - Build WASM artifacts" -ForegroundColor Gray
Write-Host "  .\scripts\generate-c-headers.ps1  - Generate C headers" -ForegroundColor Gray
Write-Host "  .\scripts\run-benchmarks.ps1      - Run benchmarks" -ForegroundColor Gray

Write-Host "`nSetup script completed!" -ForegroundColor Cyan