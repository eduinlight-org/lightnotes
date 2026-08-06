param(
  [Parameter(Mandatory = $true)]
  [string]$Path
)

$ErrorActionPreference = "Stop"

if (-not $env:WINDOWS_CERTIFICATE) {
  Write-Host "WINDOWS_CERTIFICATE is not set, leaving $Path unsigned"
  exit 0
}

function Resolve-SignTool {
  $onPath = Get-Command signtool.exe -ErrorAction SilentlyContinue
  if ($onPath) {
    return $onPath.Source
  }

  $roots = @(
    "${env:ProgramFiles(x86)}\Windows Kits\10\bin",
    "${env:ProgramFiles}\Windows Kits\10\bin"
  ) | Where-Object { $_ -and (Test-Path $_) }

  foreach ($root in $roots) {
    $found = Get-ChildItem -Path $root -Recurse -Filter signtool.exe -ErrorAction SilentlyContinue |
      Where-Object { $_.FullName -like "*\x64\*" } |
      Sort-Object FullName -Descending |
      Select-Object -First 1

    if ($found) {
      return $found.FullName
    }
  }

  throw "signtool.exe not found. Install the Windows SDK or put signtool.exe on PATH."
}

$signtool = Resolve-SignTool
$timestampUrl = if ($env:WINDOWS_TIMESTAMP_URL) { $env:WINDOWS_TIMESTAMP_URL } else { "http://timestamp.digicert.com" }

$pfx = Join-Path ([System.IO.Path]::GetTempPath()) "lightnotes-signing.pfx"

if (-not (Test-Path $pfx)) {
  [System.IO.File]::WriteAllBytes($pfx, [System.Convert]::FromBase64String($env:WINDOWS_CERTIFICATE))
}

$signArgs = @(
  "sign",
  "/f", $pfx,
  "/fd", "sha256",
  "/tr", $timestampUrl,
  "/td", "sha256"
)

if ($env:WINDOWS_CERTIFICATE_PASSWORD) {
  $signArgs += @("/p", $env:WINDOWS_CERTIFICATE_PASSWORD)
}

$signArgs += $Path

Write-Host "Signing $Path"
& $signtool @signArgs

if ($LASTEXITCODE -ne 0) {
  throw "signtool failed with exit code $LASTEXITCODE"
}
