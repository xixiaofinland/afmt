<#
.SYNOPSIS
  Installs 'afmt' CLI on Windows.

.DESCRIPTION
  1. Detects CPU architecture (x86_64 / aarch64).
  2. Fetches the latest release data from GitHub.
  3. Downloads the matching windows ZIP asset.
  4. Extracts it, moves 'afmt.exe' to a user-level directory.
  5. (Optional) Updates PATH environment variable.

#>

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

# --- CONFIGURATIONS ---

$Repo          = "xixiaofinland/afmt"
$ApiUrl        = "https://api.github.com/repos/$Repo/releases/latest"
$BinaryName    = "afmt.exe"
$InstallFolder = "$env:USERPROFILE\AppData\Local\Programs\afmt"
  # Feel free to change to your preferred location
$TempDir       = Join-Path $env:TEMP "afmt_download"

# --- FUNCTIONS ---

Function Get-Architecture {
    # Convert Windows environment arch to "x86_64" or "aarch64"
    switch ($env:PROCESSOR_ARCHITECTURE.ToLower()) {
        "amd64"   { return "x86_64" }
        "x86"     { return "x86_64" } # 32-bit fallback if needed
        "arm64"   { return "aarch64" }
        default   {
            Write-Host "Unsupported CPU architecture: $($env:PROCESSOR_ARCHITECTURE)"
            return $null
        }
    }
}

# --- MAIN SCRIPT ---

# 1) Detect architecture
$arch = Get-Architecture
if (-not $arch) {
    exit 1
}

# 2) Create temp directory
if (Test-Path $TempDir) {
    Remove-Item $TempDir -Recurse -Force
}
New-Item -ItemType Directory -Path $TempDir | Out-Null

# 3) Fetch latest release data from GitHub
Write-Host "Fetching latest release info from GitHub..."
try {
    $releaseData = Invoke-RestMethod -Uri $ApiUrl -UseBasicParsing
}
catch {
    Write-Host "Error: Unable to fetch release data."
    exit 1
}

# 4) Find the windows ZIP asset
#    Filenames look like: afmt-vX.Y.Z-windows-x86_64.zip
#    We'll match "windows-$arch"
$pattern = "windows-$arch"
Write-Host "Looking for an asset with pattern: $pattern"

$assets = $releaseData.assets | Where-Object {
    $_.browser_download_url -match $pattern -and $_.browser_download_url -like '*.zip'
}

if (-not $assets) {
    Write-Host "Error: No matching ZIP asset found for pattern '$pattern'."
    exit 1
}

# Grab the first matching asset
$assetUrl = $assets[0].browser_download_url
Write-Host "Downloading asset from: $assetUrl"

# 5) Download the ZIP file
$zipFile = Join-Path $TempDir (Split-Path $assetUrl -Leaf)
Invoke-WebRequest -Uri $assetUrl -OutFile $zipFile -UseBasicParsing

# 6) Extract the ZIP
Write-Host "Extracting ZIP to: $TempDir"
Add-Type -AssemblyName System.IO.Compression.FileSystem
[System.IO.Compression.ZipFile]::ExtractToDirectory($zipFile, $TempDir)

# 7) Locate and move 'afmt.exe'
#    If the ZIP has an enclosed folder, we'll search for 'afmt.exe'.
$afmtPath = Get-ChildItem -Path $TempDir -Filter $BinaryName -Recurse -File |
            Select-Object -First 1

if (-not $afmtPath) {
    Write-Host "Error: Could not find '$BinaryName' in the extracted files."
    exit 1
}

Write-Host "Found $($afmtPath.FullName). Moving to $InstallFolder..."

# Ensure the target folder exists
if (-not (Test-Path $InstallFolder)) {
    New-Item -ItemType Directory -Path $InstallFolder | Out-Null
}

Move-Item -Path $afmtPath.FullName -Destination (Join-Path $InstallFolder $BinaryName) -Force

# 8) Clean up
Remove-Item $TempDir -Recurse -Force

# 9) Prompt to add InstallFolder to PATH
if ($Env:PATH -notmatch [regex]::Escape($InstallFolder)) {
    Write-Host ""
    $response = Read-Host "Do you want to add '$InstallFolder' to your PATH? (y/n)"
    if ($response -match '^[Yy]') {
        try {
            [System.Environment]::SetEnvironmentVariable(
                "PATH",
                "$Env:PATH;$InstallFolder",
                [System.EnvironmentVariableTarget]::User
            )
            Write-Host "Successfully added '$InstallFolder' to your PATH."
            Write-Host "Please restart your PowerShell or open a new terminal session to apply the changes."
        }
        catch {
            Write-Host "Error: Failed to update PATH. Please add '$InstallFolder' to your PATH manually."
        }
    }
    else {
        Write-Host "Skipped adding '$InstallFolder' to PATH. You can manually add it later if needed."
    }
}

# Define color constants for better readability and maintenance
$Green = "Green"
$Yellow = "Yellow"
$Cyan = "Cyan"  # Optional: For separator lines

# Separator Line
Write-Host ""
Write-Host "========================== Complete! ==================================" -ForegroundColor $Cyan

# Installation Success Messages in Green
Write-Host "afmt installed to: $InstallFolder\$BinaryName" -ForegroundColor $Green
Write-Host ""

# Check if InstallFolder is in PATH
if ($Env:PATH -notmatch [regex]::Escape($InstallFolder)) {
    Write-Host ""
    Write-Host "========================== Note! ==================================" -ForegroundColor $Cyan
    Write-Host "$InstallFolder is not in your PATH." -ForegroundColor $Yellow
    Write-Host "Run it with full path: '$InstallFolder\$BinaryName --help'" -ForegroundColor $Yellow
} else{
    Write-Host "Run 'afmt --help' to verify." -ForegroundColor $Green
}
Write-Host ""
