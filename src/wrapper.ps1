# This is the place where it will store the saved data
$ENV:NAVTAR_DIR = "$HOME\.data\navtar-data"

function wm {
    param(
        [Parameter(ValueFromRemainingArguments = $true)]
        [string[]]$Args
    )

    $navtarExe = "path\to\navtar.exe"

    if (!(Test-Path $navtarExe)) {
        Write-Host "Executable not found: $navtarExe" -ForegroundColor Red
        return
    }

    if (-not $Args) {
        & $navtarExe --help
        return
    }

    switch ($Args[0]) {
        "list" {
            & $navtarExe list
        }
        "add" {
            if ($Args.Count -lt 3) {
                Write-Host "Usage: wm add <name> <path>" -ForegroundColor Yellow
            } else {
                & $navtarExe add $Args[1] $Args[2]
            }
        }
        "remove" {
            if ($Args.Count -lt 2) {
                Write-Host "Usage: wm remove <name>" -ForegroundColor Yellow
            } else {
                & $navtarExe remove $Args[1]
            }
        }
        "get" {
            if ($Args.Count -lt 2) {
                Write-Host "Usage: wm get <name>" -ForegroundColor Yellow
            } else {
                & $navtarExe get $Args[1]
            }
        }
        "rename" {
            if ($Args.Count -lt 3) {
                Write-Host "Usage: wm rename <old_name> <new_name>" -ForegroundColor Yellow
            } else {
                & $navtarExe rename $Args[1] $Args[2]
            }
        }
        default {
            & $navtarExe get $Args[0]
            if ($LASTEXITCODE -eq 0) {
                $target = (& $navtarExe get $Args[0]) | Select-Object -First 1
                $cleanTarget = $target.Trim('"')
                if (Test-Path $cleanTarget) {
                    Set-Location $cleanTarget
                }
            }
        }
    }
}
