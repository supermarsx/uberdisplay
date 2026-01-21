$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
$magiskScript = Join-Path $repoRoot "magisk\\uberdisplay-root\\zip-module.ps1"

& $magiskScript
