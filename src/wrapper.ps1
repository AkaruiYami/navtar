# This is the place where it will store the saved data
# NOTE: Don't forget to change the $navtarExe to the path where you put the execuble is
$ENV:NAVTAR_DIR = "$HOME\.data\navtar-data"

$navtarExe = "path\to\navtar.exe"

function wm {
    param(
        [Parameter(Position = 0)]
        [string]$Name,

        [Parameter(Position = 1)]
        [string[]]$ExtraArgs
    )

    if (!(Test-Path $navtarExe)) {
        Write-Host "Executable not found: $navtarExe" -ForegroundColor Red
        return
    }

    if (-not $Name -or $Name -in @('-h', '--help')) {
        & $navtarExe --help
        return
    }

    switch ($Name) {
        "list"    { & $navtarExe list }
        "add"     { & $navtarExe add $ExtraArgs[0] $ExtraArgs[1] }
        "remove"  { & $navtarExe remove $ExtraArgs[0] }
        "get"     { & $navtarExe get $ExtraArgs[0] }
        "rename"  { & $navtarExe rename $ExtraArgs[0] $ExtraArgs[1] }
        default {
            & $navtarExe get $Name
            if ($LASTEXITCODE -eq 0) {
                $target = (& $navtarExe get $Name) | Select-Object -First 1
                $cleanTarget = $target.Trim('"')
                if (Test-Path $cleanTarget) {
                    Set-Location $cleanTarget
                }
            }
        }
    }
}

Register-ArgumentCompleter -CommandName wm -ParameterName Name -ScriptBlock {
    param($commandName, $parameterName, $wordToComplete, $commandAst, $fakeBoundParameters)

    if (Test-Path $navtarExe) {
        & $navtarExe list |
            ForEach-Object {
                if ($_ -match "'([^']+)'") {
                    $name = $matches[1]
                    if ($name -like "$wordToComplete*") {
                        [System.Management.Automation.CompletionResult]::new($name, $name, 'ParameterValue', $name)
                    }
                }
            }
    }
}
