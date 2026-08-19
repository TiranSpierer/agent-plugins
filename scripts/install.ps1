<#
.SYNOPSIS
  Automated Installer for Antigravity & Claude Code Custom Statusline Plugin (Windows)
#>

$ErrorActionPreference = "Stop"

Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "  Antigravity CLI Custom Statusline - Fast Installer        " -ForegroundColor Green
Write-Host "============================================================" -ForegroundColor Cyan

$userHome = [Environment]::GetFolderPath("UserProfile")
$targetPluginDir = Join-Path $userHome ".gemini\antigravity-cli\plugins\custom-statusline"
$targetBinDir = Join-Path $targetPluginDir "bin"
$targetSkillsDir = Join-Path $targetPluginDir "skills\custom-statusline"

New-Item -ItemType Directory -Path $targetBinDir -Force | Out-Null
New-Item -ItemType Directory -Path $targetSkillsDir -Force | Out-Null

$scriptDir = $PSScriptRoot
$repoRoot = if ($scriptDir) { Split-Path $scriptDir -Parent } else { $null }

$installedExe = Join-Path $targetBinDir "statusline.exe"

if ($repoRoot -and (Test-Path (Join-Path $repoRoot "plugins\custom-statusline\bin\statusline.exe"))) {
    Write-Host "[1/4] Copying binary from local repository..." -ForegroundColor Yellow
    Copy-Item -Path (Join-Path $repoRoot "plugins\custom-statusline\bin\statusline.exe") -Destination $installedExe -Force
    Copy-Item -Path (Join-Path $repoRoot "plugins\custom-statusline\plugin.json") -Destination (Join-Path $targetPluginDir "plugin.json") -Force
    Copy-Item -Path (Join-Path $repoRoot "plugins\custom-statusline\skills\custom-statusline\SKILL.md") -Destination (Join-Path $targetSkillsDir "SKILL.md") -Force
} elseif (Get-Command cargo -ErrorAction SilentlyContinue) {
    Write-Host "[1/4] Compiling statusline from Rust source..." -ForegroundColor Yellow
    $srcDir = if ($repoRoot) { Join-Path $repoRoot "plugins\custom-statusline\src" } else { "." }
    Push-Location $srcDir
    cargo build --release
    Pop-Location
    Copy-Item -Path (Join-Path $srcDir "target\release\statusline.exe") -Destination $installedExe -Force
} else {
    Write-Host "[1/4] Downloading latest pre-compiled binary from GitHub releases..." -ForegroundColor Yellow
    $releaseUrl = "https://github.com/TiranSpierer/antigravity-plugins/releases/latest/download/statusline-windows-x86_64.exe"
    Invoke-WebRequest -Uri $releaseUrl -OutFile $installedExe
}

Write-Host "[2/4] Verifying binary execution..." -ForegroundColor Yellow
& $installedExe --demo

Write-Host "[3/4] Updating Antigravity CLI configuration..." -ForegroundColor Yellow
$settingsPaths = @(
    (Join-Path $userHome ".gemini\antigravity-cli\settings.json"),
    (Join-Path $userHome ".gemini\settings.json")
)

foreach ($sPath in $settingsPaths) {
    if (Test-Path $sPath) {
        try {
            $json = Get-Content $sPath -Raw | ConvertFrom-Json
            if (-not $json.statusLine) {
                $json | Add-Member -NotePropertyName "statusLine" -NotePropertyValue ([PSCustomObject]@{})
            }
            $json.statusLine.type = "command"
            $json.statusLine.command = $installedExe
            $json.statusLine.enabled = $true

            $json | ConvertTo-Json -Depth 10 | Set-Content $sPath -Encoding UTF8
            Write-Host "  Updated $sPath" -ForegroundColor Green
        } catch {
            Write-Warning "Could not update $sPath: $_"
        }
    }
}

Write-Host "[4/4] Validating plugin..." -ForegroundColor Yellow
if (Get-Command agy -ErrorAction SilentlyContinue) {
    agy plugin validate $targetPluginDir
}

Write-Host "`nCustom Statusline successfully installed & activated!" -ForegroundColor Green
Write-Host "Restart or open your Antigravity CLI to enjoy the new statusline." -ForegroundColor Cyan
