[CmdletBinding()]
param(
    [switch]$SkipContentBuild,
    [switch]$CheckOnly
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$Root = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $Root

function Require-Command {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Name,
        [Parameter(Mandatory = $true)]
        [string]$InstallHint
    )

    if (-not (Get-Command $Name -ErrorAction SilentlyContinue)) {
        throw "Missing command '$Name'. $InstallHint"
    }
}

function Invoke-ExternalStep {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Title,
        [Parameter(Mandatory = $true)]
        [scriptblock]$Command
    )

    Write-Host ""
    Write-Host "==> $Title"
    & $Command
    if ($LASTEXITCODE -ne 0) {
        throw "$Title failed with exit code $LASTEXITCODE."
    }
}

Write-Host "[RebrnG] Workspace: $Root"
Require-Command -Name "pnpm" -InstallHint "Enable Corepack and run: corepack prepare pnpm@10.33.2 --activate"
Require-Command -Name "cargo" -InstallHint "Install Rust stable via rustup, then reopen the terminal."

if ($CheckOnly) {
    Write-Host "[RebrnG] Toolchain check passed. Use .\start-game.cmd to launch the desktop dev build."
    exit 0
}

if (-not $SkipContentBuild) {
    Invoke-ExternalStep -Title "Build S0 content bundle" -Command {
        pnpm content:build
    }
}

Write-Host ""
Write-Host "==> Start RebrnG desktop dev build"
Write-Host "[RebrnG] This keeps a terminal open for Vite/Tauri logs. Close the game window or press Ctrl+C here to stop."
pnpm desktop:dev
exit $LASTEXITCODE
