# GSLC Installation Script for Windows
# Usage: iwr -useb https://raw.githubusercontent.com/politikl/gslc/main/install.ps1 | iex

Write-Host "🚀 Installing GSLC - Geometry Shorthand Language Compiler" -ForegroundColor Cyan
Write-Host ""

$ErrorActionPreference = "Stop"

# Get latest release
Write-Host "📦 Fetching latest release..." -ForegroundColor Yellow
try {
    $response = Invoke-RestMethod -Uri "https://api.github.com/repos/politikl/gslc/releases/latest"
    $version = $response.tag_name
    Write-Host "   Latest version: $version" -ForegroundColor Green
}
catch {
    Write-Host "❌ Failed to fetch latest release" -ForegroundColor Red
    exit 1
}

# Download URL
$filename = "gslc-windows-x86_64.zip"
$downloadUrl = "https://github.com/politikl/gslc/releases/download/$version/$filename"

Write-Host "⬇️  Downloading GSLC..." -ForegroundColor Yellow
Write-Host "   URL: $downloadUrl"

# Create temp directory
$tempDir = New-Item -ItemType Directory -Path (Join-Path $env:TEMP ([System.IO.Path]::GetRandomFileName()))
$downloadPath = Join-Path $tempDir $filename

try {
    Invoke-WebRequest -Uri $downloadUrl -OutFile $downloadPath
}
catch {
    Write-Host "❌ Download failed" -ForegroundColor Red
    Remove-Item -Recurse -Force $tempDir
    exit 1
}

# Extract
Write-Host "📂 Extracting..." -ForegroundColor Yellow
Expand-Archive -Path $downloadPath -DestinationPath $tempDir -Force

# Install directory
$installDir = "$env:USERPROFILE\.local\bin"
if (-not (Test-Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir -Force | Out-Null
}

# Copy binary
Write-Host "💾 Installing..." -ForegroundColor Yellow
$binaryPath = Join-Path $tempDir "gslc.exe"
$installPath = Join-Path $installDir "gslc.exe"

Copy-Item -Path $binaryPath -Destination $installPath -Force

# Cleanup
Remove-Item -Recurse -Force $tempDir

Write-Host ""
Write-Host "✅ GSLC installed successfully!" -ForegroundColor Green
Write-Host ""
Write-Host "📍 Installed to: $installPath"
Write-Host ""

# Check if in PATH
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -like "*$installDir*") {
    Write-Host "✓ $installDir is in your PATH" -ForegroundColor Green
}
else {
    Write-Host "⚠️  Adding $installDir to your PATH..." -ForegroundColor Yellow
    $newPath = "$userPath;$installDir"
    [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
    Write-Host "✓ PATH updated. Please restart your terminal." -ForegroundColor Green
}

Write-Host ""
Write-Host "🎉 Try it out:" -ForegroundColor Cyan
Write-Host "  gslc '\\P:A/P:B/S:AB\\'"
Write-Host ""
Write-Host "📚 Documentation: https://tinyurl.com/geoshorthand" -ForegroundColor Cyan
