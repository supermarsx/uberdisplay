$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
$pcAppDir = Join-Path $repoRoot "pc\\app"
$pcTauriDir = Join-Path $repoRoot "pc\\src-tauri"

Push-Location $pcAppDir
try {
    if (-not (Test-Path (Join-Path $pcAppDir "node_modules"))) {
        npm install
    }
    npm run build
} finally {
    Pop-Location
}

Push-Location $pcTauriDir
try {
    cargo tauri build
} finally {
    Pop-Location
}
