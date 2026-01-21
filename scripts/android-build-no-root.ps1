$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
$androidDir = Join-Path $repoRoot "android"

Push-Location $androidDir
try {
    .\gradlew.bat assembleDebug
} finally {
    Pop-Location
}
