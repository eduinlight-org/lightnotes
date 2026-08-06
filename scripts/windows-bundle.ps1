$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot

$version = if ($env:VERSION) { $env:VERSION } else { "dev" }
$target = if ($env:TARGET) { $env:TARGET } else { "x86_64-pc-windows-msvc" }
$outDir = if ($env:OUT_DIR) { $env:OUT_DIR } else { Join-Path $root "dist\windows" }

$arch = if ($env:ARCH) { $env:ARCH } else { "x86_64" }
switch ($arch) {
  { $_ -in "x86_64", "amd64", "x64" } { $arch = "x86_64" }
  { $_ -in "aarch64", "arm64" } { $arch = "aarch64" }
  default { throw "unsupported architecture: $arch" }
}

if (-not $env:WINDOWS_CERTIFICATE) {
  Write-Host "::warning::WINDOWS_CERTIFICATE is not set, producing unsigned installers"
}

$base = "LightNotes-$version-windows-$arch"

New-Item -ItemType Directory -Force -Path $outDir | Out-Null
Get-ChildItem -Path (Join-Path $outDir "*") -Include *.msi, *.exe -File -ErrorAction SilentlyContinue | Remove-Item -Force

Push-Location $root
try {
  dx bundle `
    --package desktop `
    --platform windows `
    --release `
    --target $target `
    --package-types msi `
    --package-types nsis `
    --out-dir $outDir

  if ($LASTEXITCODE -ne 0) {
    throw "dx bundle failed with exit code $LASTEXITCODE"
  }
}
finally {
  Pop-Location
}

$msi = Get-ChildItem -Path $outDir -Filter *.msi -File | Select-Object -First 1
$setup = Get-ChildItem -Path $outDir -Filter *-setup.exe -File | Select-Object -First 1

if (-not $msi) {
  Get-ChildItem -Path $outDir | Format-Table -AutoSize | Out-String | Write-Host
  throw "dx bundle produced no .msi in $outDir"
}
if (-not $setup) {
  Get-ChildItem -Path $outDir | Format-Table -AutoSize | Out-String | Write-Host
  throw "dx bundle produced no NSIS -setup.exe in $outDir"
}

Move-Item -Force $msi.FullName (Join-Path $outDir "$base.msi")
Move-Item -Force $setup.FullName (Join-Path $outDir "$base-setup.exe")

Get-ChildItem -Path $outDir | Format-Table -AutoSize | Out-String | Write-Host
