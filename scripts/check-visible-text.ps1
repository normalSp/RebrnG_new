$ErrorActionPreference = "Stop"

$Root = Split-Path -Parent $PSScriptRoot
$TargetRoots = @(
    "crates/game-core/src",
    "packages/ui-ledger/src",
    "apps/desktop/src",
    "apps/desktop/src-tauri/src",
    "content/s0",
    "docs/superpowers/specs",
    "docs/superpowers/plans",
    "docs/superpowers/reviews"
)
$AllowedExtensions = @(".rs", ".ts", ".tsx", ".css", ".yaml", ".yml", ".md")
$SkippedPathFragments = @("\node_modules\", "\target\", "\dist\", "\assets\")

$ForbiddenFragments = @(
    @{ Name = "U+FFFD replacement character"; Value = [string][char]0xFFFD },
    @{ Name = "known mojibake marker U+95C8"; Value = [string][char]0x95C8 },
    @{ Name = "known mojibake marker U+947A"; Value = [string][char]0x947A },
    @{ Name = "known mojibake marker U+7490"; Value = [string][char]0x7490 },
    @{ Name = "known mojibake marker U+9365"; Value = [string][char]0x9365 },
    @{ Name = "known mojibake marker U+94D4"; Value = [string][char]0x94D4 },
    @{ Name = "known mojibake marker U+947E"; Value = [string][char]0x947E },
    @{ Name = "known mojibake marker U+5A13"; Value = [string][char]0x5A13 },
    @{ Name = "known mojibake marker U+699B"; Value = [string][char]0x699B },
    @{ Name = "known mojibake marker U+7EC9"; Value = [string][char]0x7EC9 },
    @{ Name = "known mojibake marker U+947D"; Value = [string][char]0x947D },
    @{ Name = "legacy mojibake compatibility marker U+535E"; Value = [string][char]0x535E },
    @{ Name = "known mojibake marker U+59DD"; Value = [string][char]0x59DD },
    @{ Name = "known mojibake marker U+5FAD"; Value = [string][char]0x5FAD },
    @{ Name = "known mojibake marker U+6769"; Value = [string][char]0x6769 },
    @{ Name = "known mojibake marker U+8926"; Value = [string][char]0x8926 },
    @{ Name = "known mojibake marker U+701B"; Value = [string][char]0x701B },
    @{ Name = "known mojibake marker U+93C3"; Value = [string][char]0x93C3 },
    @{ Name = "known mojibake marker U+93C7"; Value = [string][char]0x93C7 },
    @{ Name = "known mojibake marker U+93BB"; Value = [string][char]0x93BB },
    @{ Name = "known mojibake marker U+7487"; Value = [string][char]0x7487 },
    @{ Name = "known mojibake marker U+7459"; Value = [string][char]0x7459 },
    @{ Name = "known mojibake marker U+6434"; Value = [string][char]0x6434 },
    @{ Name = "known mojibake marker U+7035"; Value = [string][char]0x7035 },
    @{ Name = "known mojibake marker U+93AC"; Value = [string][char]0x93AC },
    @{ Name = "known mojibake marker U+93C2"; Value = [string][char]0x93C2 },
    @{ Name = "known mojibake marker U+934A"; Value = [string][char]0x934A },
    @{ Name = "known mojibake marker U+7ECC"; Value = [string][char]0x7ECC },
    @{ Name = "known mojibake marker U+9417"; Value = [string][char]0x9417 },
    @{ Name = "known mojibake marker U+6DC7"; Value = [string][char]0x6DC7 },
    @{ Name = "known mojibake marker U+59F9"; Value = [string][char]0x59F9 },
    @{ Name = "known mojibake marker U+59AB"; Value = [string][char]0x59AB },
    @{ Name = "known mojibake marker U+9480"; Value = [string][char]0x9480 }
)

$Hits = New-Object System.Collections.Generic.List[string]

foreach ($target in $TargetRoots) {
    $targetPath = Join-Path $Root $target
    if (-not (Test-Path $targetPath)) {
        continue
    }

    Get-ChildItem -Path $targetPath -Recurse -File | ForEach-Object {
        $file = $_
        if ($AllowedExtensions -notcontains $file.Extension) {
            return
        }

        $normalized = $file.FullName.Replace("/", "\")
        foreach ($fragment in $SkippedPathFragments) {
            if ($normalized.Contains($fragment)) {
                return
            }
        }

        $raw = Get-Content -LiteralPath $file.FullName -Raw -Encoding UTF8
        if (($file.Extension -eq ".yaml" -or $file.Extension -eq ".yml") -and $normalized.Contains("\content\")) {
            $unicodeEscape = [regex]::Match($raw, '\\u[0-9A-Fa-f]{4}')
            if ($unicodeEscape.Success) {
                $line = ($raw.Substring(0, $unicodeEscape.Index).Split("`n")).Count
                $relative = $file.FullName
                if ($relative.StartsWith($Root)) {
                    $relative = $relative.Substring($Root.Length).TrimStart("\", "/")
                }
                $Hits.Add("${relative}:${line} contains Unicode escape; content YAML should use raw UTF-8 text")
            }
        }

        foreach ($forbidden in $ForbiddenFragments) {
            if ($raw.Contains($forbidden.Value)) {
                $index = $raw.IndexOf($forbidden.Value)
                $line = ($raw.Substring(0, $index).Split("`n")).Count
                $relative = $file.FullName
                if ($relative.StartsWith($Root)) {
                    $relative = $relative.Substring($Root.Length).TrimStart("\", "/")
                }
                $Hits.Add("${relative}:${line} contains $($forbidden.Name)")
            }
        }
    }
}

if ($Hits.Count -gt 0) {
    Write-Host "Visible text mojibake check failed:" -ForegroundColor Red
    $Hits | ForEach-Object { Write-Host " - $_" -ForegroundColor Red }
    exit 1
}

Write-Host "Visible text mojibake check passed." -ForegroundColor Green
