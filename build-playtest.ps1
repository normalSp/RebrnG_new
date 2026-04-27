[CmdletBinding()]
param(
    [switch]$SkipContentBuild
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

function Invoke-Step {
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

function Show-PlaytestArtifacts {
    $artifactFiles = @()
    $releaseExe = Join-Path $Root "target\release\rebrng-desktop.exe"
    if (Test-Path $releaseExe) {
        $artifactFiles += Get-Item $releaseExe
    }

    $bundleRoots = @(
        (Join-Path $Root "target\release\bundle"),
        (Join-Path $Root "apps\desktop\src-tauri\target\release\bundle")
    ) | Where-Object { Test-Path $_ }

    foreach ($bundleRoot in $bundleRoots) {
        $artifactFiles += Get-ChildItem -Path $bundleRoot -Recurse -File |
            Sort-Object LastWriteTime -Descending |
            Select-Object -First 12
    }

    if ($artifactFiles.Count -eq 0) {
        Write-Host "[RebrnG] No known playtest artifacts were found." -ForegroundColor Yellow
        return
    }

    Write-Host ""
    Write-Host "[RebrnG] Playtest artifacts or partial artifacts:" -ForegroundColor Green
    foreach ($artifactFile in $artifactFiles) {
        Write-Host (" - " + $artifactFile.FullName)
    }
}

Write-Host "[RebrnG] Playtest build workspace: $Root"
Require-Command -Name "pnpm" -InstallHint "Enable Corepack and run: corepack prepare pnpm@10.33.2 --activate"
Require-Command -Name "cargo" -InstallHint "Install Rust stable via rustup, then reopen the terminal."

try {
    if (-not $SkipContentBuild) {
        Invoke-Step -Title "Build S0 content bundle" -Command { pnpm content:build }
    }

    Invoke-Step -Title "Build Tauri desktop playtest package" -Command { pnpm desktop:build }
    Show-PlaytestArtifacts
} catch {
    Write-Host ""
    Write-Host "[RebrnG] Playtest build blocked." -ForegroundColor Red
    Write-Host $_.Exception.Message -ForegroundColor Red
    Show-PlaytestArtifacts
    Write-Host "Common Windows blockers: missing WebView2, missing Visual Studio C++ Build Tools, missing WiX/NSIS bundler dependencies, or unavailable network for Tauri tooling." -ForegroundColor Yellow
    exit 1
}
