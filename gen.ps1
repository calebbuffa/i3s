# Regenerate i3s Rust types from the I3S spec.
#
# NORMAL BUILD: generated Rust source files are committed; running this script
# is only necessary when the spec changes.
#
# TWO-STEP PIPELINE:
#   Step 1 (optional — needs i3s-spec submodule):
#     python scripts/parse_spec.py
#       Reads  extern/i3s-spec/**/*.md
#       Writes generated/i3s_spec.json
#
#   Step 2 (needs only Python + the committed generated/i3s_spec.json):
#     python scripts/generate_rust.py
#       Reads  generated/i3s_spec.json
#       Writes i3s/src/{cmn,bld,pcsl,psl,...}.rs

$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $MyInvocation.MyCommand.Path

$parseSpec    = Join-Path $root "scripts\parse_spec.py"
$generateRust = Join-Path $root "scripts\generate_rust.py"
$specDir      = Join-Path $root "extern\i3s-spec"
$ir           = Join-Path $root "generated\i3s_spec.json"

if ($args -contains "--full" -and (Test-Path $specDir)) {
    Write-Host "Step 1: parsing i3s-spec markdown -> JSON IR" -ForegroundColor Cyan
    python $parseSpec --spec-dir $specDir --output $ir
}

Write-Host "Step 2: generating Rust from JSON IR" -ForegroundColor Cyan
python $generateRust --ir $ir --output (Join-Path $root "i3s\src")
Write-Host "Done." -ForegroundColor Green
