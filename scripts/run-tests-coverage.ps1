# Save this as scripts/run-tests-coverage.ps1

Write-Host "=== Running Tests with Coverage ===" -ForegroundColor Cyan

# Clean previous coverage
Remove-Item -Path "tarpaulin-report.html" -ErrorAction SilentlyContinue
Remove-Item -Path "cobertura.xml" -ErrorAction SilentlyContinue

Write-Host "Running tests with coverage..." -ForegroundColor Yellow
cargo tarpaulin --verbose `
    --all-features `
    --workspace `
    --timeout 120 `
    --out Html `
    --out Xml `
    --output-dir . `
    --exclude-files "*/benches/*" `
    --exclude-files "*/tests/*" `
    --ignore-panics `
    --ignore-tests

Write-Host "`nCoverage report generated!" -ForegroundColor Green
Write-Host "HTML Report: tarpaulin-report.html" -ForegroundColor White
Write-Host "XML Report: cobertura.xml" -ForegroundColor White

# Open HTML report in browser
Start-Process "tarpaulin-report.html"

# Display coverage summary
Write-Host "`nCoverage Summary:" -ForegroundColor Cyan
cargo tarpaulin --print-summary