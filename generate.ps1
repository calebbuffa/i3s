# Regenerate i3s Rust types from the I3S spec.
#
# NORMAL BUILD: generated Rust source files are committed; running this script
# is only necessary when the spec changes.
#
# JSON Schema source: extern/i3s-schema/schema/
# The i3s-schema project owns Markdown-to-JSON Schema conversion.

$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $MyInvocation.MyCommand.Path

$schemaDir = Join-Path $root "extern\i3s-schema\schema"

Push-Location $root
try {
    Write-Host "Generating Rust from external JSON Schema" -ForegroundColor Cyan
    cargo run -q -p xtask -- --schemas $schemaDir --output src\generated.rs
    if ($LASTEXITCODE -ne 0) { throw "i3s generation failed with exit code $LASTEXITCODE" }
    Write-Host "Done." -ForegroundColor Green
}
finally { Pop-Location }
