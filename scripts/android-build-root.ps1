$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
$androidDir = Join-Path $repoRoot "android"
$magiskScript = Join-Path $repoRoot "magisk\\uberdisplay-root\\zip-module.ps1"

Push-Location $androidDir
try {
    .\gradlew.bat assembleDebug
} finally {
    Pop-Location
}

& $magiskScript
