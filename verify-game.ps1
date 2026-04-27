[CmdletBinding()]
param()

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

function Assert-NoSearchHits {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Title,
        [Parameter(Mandatory = $true)]
        [string[]]$Paths,
        [Parameter(Mandatory = $true)]
        [string[]]$Patterns,
        [string[]]$Includes = @("*.rs", "*.ts", "*.tsx", "*.yaml", "*.yml", "*.json")
    )

    Write-Host ""
    Write-Host "==> $Title"
    $hits = @()
    foreach ($pattern in $Patterns) {
        $matches = Get-ChildItem -Path $Paths -Recurse -File -Include $Includes |
            Select-String -Pattern $pattern -SimpleMatch
        if ($matches) {
            $hits += $matches
        }
    }

    if ($hits.Count -gt 0) {
        $hits | ForEach-Object {
            Write-Host ("{0}:{1}:{2}" -f $_.Path, $_.LineNumber, $_.Line.Trim()) -ForegroundColor Red
        }
        throw "$Title found forbidden references."
    }

    Write-Host "No forbidden references found."
}

Write-Host "[RebrnG] Verification workspace: $Root"
Require-Command -Name "pnpm" -InstallHint "Enable Corepack and run: corepack prepare pnpm@10.33.2 --activate"
Require-Command -Name "cargo" -InstallHint "Install Rust stable via rustup, then reopen the terminal."

Invoke-Step -Title "Startup entry check" -Command { .\start-game.cmd -CheckOnly }
Invoke-Step -Title "Visible text mojibake check" -Command {
    powershell.exe -NoProfile -ExecutionPolicy Bypass -File .\scripts\check-visible-text.ps1
}
Invoke-Step -Title "Rust format check" -Command { cargo fmt --check }
Invoke-Step -Title "Rust clippy" -Command { cargo clippy --workspace --all-targets -- -D warnings }
Invoke-Step -Title "Rust tests" -Command { cargo test --workspace }
Invoke-Step -Title "Desktop Rust check" -Command { cargo check -p rebrng-desktop }
Invoke-Step -Title "Content bundle build" -Command { pnpm content:build }
Invoke-Step -Title "Frontend build" -Command { pnpm -r build }

Assert-NoSearchHits `
    -Title "Runtime AI and Express red-line check" `
    -Paths @("crates", "apps/desktop/src", "packages/ui-ledger/src", "content") `
    -Patterns @("Express", "runtime AI proposal", "runtime AI narrator", "proposal", "narrator")

Assert-NoSearchHits `
    -Title "React full GameState boundary check" `
    -Paths @("apps/desktop/src", "packages/ui-ledger/src") `
    -Patterns @("GameState") `
    -Includes @("*.ts", "*.tsx")

Assert-NoSearchHits `
    -Title "Frontend content-source scan boundary check" `
    -Paths @("apps/desktop/src", "packages/ui-ledger/src") `
    -Patterns @("yaml", "YAML", "readFile", "fs.") `
    -Includes @("*.ts", "*.tsx")

Write-Host ""
Write-Host "[RebrnG] Verification passed." -ForegroundColor Green
