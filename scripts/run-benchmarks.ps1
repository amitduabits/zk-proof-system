# Save this as scripts/run-benchmarks.ps1

Write-Host "=== Running Benchmarks ===" -ForegroundColor Cyan

# Create results directory
$resultsDir = "benchmark-results"
New-Item -ItemType Directory -Force -Path $resultsDir | Out-Null

# Set timestamp for results
$timestamp = Get-Date -Format "yyyy-MM-dd_HH-mm-ss"
$resultFile = "$resultsDir/bench_$timestamp.txt"

Write-Host "Running benchmarks for all modules..." -ForegroundColor Yellow

# Run benchmarks and save results
$benchOutput = cargo bench --all-features 2>&1 | Tee-Object -FilePath $resultFile

Write-Host "`nBenchmark results saved to: $resultFile" -ForegroundColor Green

# Parse and display summary
Write-Host "`n=== Benchmark Summary ===" -ForegroundColor Cyan

$benchOutput | Select-String -Pattern "time:|throughput:" | ForEach-Object {
    Write-Host $_.Line -ForegroundColor White
}

# Generate comparison if previous results exist
$previousResults = Get-ChildItem "$resultsDir/bench_*.txt" | Sort-Object LastWriteTime -Descending | Select-Object -Skip 1 -First 1

if ($previousResults) {
    Write-Host "`nComparing with previous run: $($previousResults.Name)" -ForegroundColor Yellow
    cargo bench --all-features -- --baseline previous
}

Write-Host "`nBenchmarks complete!" -ForegroundColor Green