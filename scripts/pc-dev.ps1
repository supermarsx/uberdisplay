$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
$pcAppDir = Join-Path $repoRoot "pc\\app"
$pcTauriDir = Join-Path $repoRoot "pc\\src-tauri"

Push-Location $pcAppDir
try {
    if (-not (Test-Path (Join-Path $pcAppDir "node_modules"))) {
        npm install
    }
} finally {
    Pop-Location
}

Start-Process -FilePath "powershell" -ArgumentList "-NoExit", "-Command", "cd `"$pcAppDir`"; npm run dev"
Start-Sleep -Seconds 2

Push-Location $pcTauriDir
try {
    cargo tauri dev
} finally {
    Pop-Location
}
