#!/usr/bin/env pwsh
<#
.SYNOPSIS
  Build and install the `smn_mora` tool for Windows or Linux.
#>

# 1. Build in release mode
Write-Host "Building smn_mora in release mode..."
& cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Error "Cargo build failed. Aborting."
    exit 1
}

# 2. Determine OS via .NET
Add-Type -AssemblyName System.Runtime
$osWin   = [System.Runtime.InteropServices.RuntimeInformation]::IsOSPlatform([System.Runtime.InteropServices.OSPlatform]::Windows)
$osLinux = [System.Runtime.InteropServices.RuntimeInformation]::IsOSPlatform([System.Runtime.InteropServices.OSPlatform]::Linux)

# Optional sanity check
# Write-Host "`$osWin = $osWin; `$osLinux = $osLinux"

if ($osWin) {
    # Windows install
    $exeName  = 'smn_mora.exe'
    $srcPath  = Join-Path -Path (Get-Location) -ChildPath "target\release\$exeName"
    $destDir  = 'C:\Summon\Tools\System Tools'
    $destPath = Join-Path -Path $destDir -ChildPath $exeName

    if (-not (Test-Path -LiteralPath $srcPath)) {
        Write-Error "Built executable not found at '$srcPath'"
        exit 1
    }

    Write-Host "Installing on Windows to '$destDir'..."
    if (-not (Test-Path -LiteralPath $destDir)) {
        Write-Host "Creating directory '$destDir'..."
        New-Item -LiteralPath $destDir -ItemType Directory -Force | Out-Null
    }

    Write-Host "Copying to '$destPath'..."
    Copy-Item -LiteralPath $srcPath -Destination $destPath -Force

    Write-Host "Installed smn_mora at '$destPath'"
    exit 0
}
elseif ($osLinux) {
    # Linux install
    $binName  = 'smn_mora'
    $srcPath  = Join-Path -Path (Get-Location) -ChildPath "target/release/$binName"
    $destDir  = '/usr/local/bin'
    $destPath = Join-Path -Path $destDir -ChildPath $binName

    if (-not (Test-Path -LiteralPath $srcPath)) {
        Write-Error "Built binary not found at '$srcPath'"
        exit 1
    }

    Write-Host "Installing on Linux to '$destDir' (sudo may be required)..."
    Write-Host "Ensuring directory exists..."
    sudo mkdir -p $destDir

    Write-Host "Copying to '$destPath'..."
    sudo cp $srcPath $destPath

    Write-Host "Setting executable permission..."
    sudo chmod +x $destPath

    Write-Host "Installed smn_mora at '$destPath'"
    exit 0
}

Write-Error "Unsupported platform. This script only supports Windows and Linux."
exit 1
