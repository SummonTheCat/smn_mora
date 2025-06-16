# Collect-RsFiles.ps1
# Usage: .\Collect-RsFiles.ps1

# 1. Get a timestamp for the output filename
$timestamp = Get-Date -Format "yyyyMMdd_HHmmss"

# 2. Ensure the 'collects' directory exists
$collectDir = Join-Path -Path (Get-Location) -ChildPath "collects"
if (-not (Test-Path -Path $collectDir)) {
    New-Item -Path $collectDir -ItemType Directory | Out-Null
}

# 3. Define output file inside the collects directory
$outFile = Join-Path -Path $collectDir -ChildPath "collect_$timestamp.clt"

# 4. Create or overwrite the output file
New-Item -Path $outFile -ItemType File -Force | Out-Null

# 5. If there's a src folder, recurse into it; otherwise exit with a message
$srcPath = Join-Path -Path (Get-Location) -ChildPath "src"
if (-not (Test-Path -Path $srcPath)) {
    Write-Error "No 'src' directory found in $(Get-Location)."
    exit 1
}

# 6. Recursively find all .rs files under src and append them to the output
Get-ChildItem -Path $srcPath -Recurse -Filter *.rs | ForEach-Object {

    # Write a header with the file’s full path
    Add-Content -Path $outFile -Value "===== File: $($_.FullName) ====="

    # Append the file’s content
    Get-Content -Path $_.FullName | Add-Content -Path $outFile

    # Add a blank line separator
    Add-Content -Path $outFile -Value ""
}

Write-Host "Collected all .rs files from 'src' into:`n  $outFile"
