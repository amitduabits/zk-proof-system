# Save this as scripts/build-wasm.ps1

Write-Host "=== Building WASM Artifacts ===" -ForegroundColor Cyan

# Check if wasm-pack is installed
if (-not (Get-Command wasm-pack -ErrorAction SilentlyContinue)) {
    Write-Host "Installing wasm-pack..." -ForegroundColor Yellow
    cargo install wasm-pack
}

# Create wasm output directory
$wasmDir = "wasm-artifacts"
New-Item -ItemType Directory -Force -Path $wasmDir | Out-Null

Write-Host "Building WASM for bindings module..." -ForegroundColor Yellow

# Build WASM package
Push-Location bindings
try {
    wasm-pack build --target web --out-dir "../$wasmDir" --release
    Write-Host "WASM build successful!" -ForegroundColor Green
} catch {
    Write-Host "WASM build failed: $_" -ForegroundColor Red
    exit 1
} finally {
    Pop-Location
}

Write-Host "`nWASM artifacts generated in: $wasmDir/" -ForegroundColor Green
Write-Host "Files created:" -ForegroundColor White
Get-ChildItem $wasmDir | ForEach-Object { Write-Host "  - $($_.Name)" -ForegroundColor Gray }

# Generate example HTML file for testing
$htmlContent = @'
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>ZK Proof System WASM Test</title>
</head>
<body>
    <h1>ZK Proof System WASM Interface</h1>
    <script type="module">
        import init, { create_proof, verify_proof } from './wasm-artifacts/zk_proof_bindings.js';
        
        async function run() {
            await init();
            
            // Test proof creation
            const input = new Uint8Array([1, 2, 3, 4]);
            const proof = create_proof(input);
            console.log('Proof created:', proof);
            
            // Test verification
            const isValid = verify_proof(proof);
            console.log('Proof valid:', isValid);
        }
        
        run();
    </script>
</body>
</html>
'@

$htmlContent | Out-File -Encoding UTF8 "$wasmDir/test.html"
Write-Host "`nTest HTML created: $wasmDir/test.html" -ForegroundColor Cyan