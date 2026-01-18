$ErrorActionPreference = "Stop"

$moduleRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$output = Join-Path $moduleRoot "uberdisplay-root.zip"

if (Test-Path $output) {
    Remove-Item -Force $output
}

Compress-Archive -Path (Join-Path $moduleRoot "*") -DestinationPath $output
Write-Host "Created $output"
