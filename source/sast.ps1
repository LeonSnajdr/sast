Param(
    [parameter(Mandatory = $false)]
    [alias("t")]
    [string] $template,

    [parameter(Mandatory = $false, ValueFromRemainingArguments = $true)]
    [alias("f")]
    [string[]] $flags
)

$scriptPath = split-path -parent $MyInvocation.MyCommand.Definition

function PrintLogo {
    Clear-Host 
    Write-Host "   _____                  _____ __             __ " 
    Write-Host "  / ___/____ _____ ___   / ___// /_____ ______/ /_"
    Write-Host "  \__ \/ __ '/ __ '__ \  \__ \/ __/ __ '/ ___/ __/"
    Write-Host " ___/ / /_/ / / / / / / ___/ / /_/ /_/ / /  / /_  "
    Write-Host "/____/\__,_/_/ /_/ /_(_)____/\__/\__,_/_/   \__/  "
    Write-Host "                                                  "
    Write-Host "Author      : " -NoNewline
    Write-Host "Leon Snajdr" -ForegroundColor Cyan
    Write-Host "Version     : v1.1.1"
}

function LoadTemplate($templatePath) {

    $template = GetReplacedTemplateJson $templatePath | ConvertFrom-Json
    $tabsStarted = @{}

    PrintLogo
    Write-Host "Template    :" $template.name
    Write-Host

    foreach ($task in $template.tasks) {
        if ($task.flags) {
            if ($null -eq $flags) {
                continue;
            }

            $containsOne = $false
            foreach ($flag in $task.flags) {
                if ($flags.Contains($flag) -eq $true) {
                    $containsOne = $true
                    break;
                }
            }

            if ($containsOne -eq $false) {
                continue;
            }
        }

        if ($task.terminalWindow) {
            Write-Host "Opening Tab :" $task.tabTitle -ForegroundColor DarkGray
            wt -w $task.terminalWindow nt -p "Windows PowerShell" --title $task.tabTitle -d $task.workingDirectory powershell -noExit $task.command

            $tabsStarted[$task.terminalWindow] ++;
            if ($tabsStarted[$task.terminalWindow] -eq 1) {
                Start-Sleep -Seconds 1.5
            }

            Start-Sleep 0.5
            continue;
        }
        
        Write-Host "Command     :" $task.command -ForegroundColor DarkGray

        Push-Location $task.workingDirectory
        Invoke-Expression -Command $task.command > $null
        Pop-Location
    }

    Write-Host
    Write-Host "All tasks executed successfully, I wish a pleasant work" -ForegroundColor Green
    Write-Host
}

function GetReplacedTemplateJson($templatePath) {
    $rawTemplate = Get-Content $templatePath
    $placeholders = ($rawTemplate | ConvertFrom-Json).placeholders.psobject.Members | where-object membertype -like 'noteproperty'

    foreach ($placeholder in $placeholders) {
        $key = $placeholder.name
        $rawTemplate = $rawTemplate.Replace("{$key}", $placeholder.Value)
    }

    return $rawTemplate
}


if (-not [string]::IsNullOrWhiteSpace($template)) {
    if ([System.IO.File]::Exists("$scriptPath\templates\own\$template.json")) {
        LoadTemplate "$scriptPath\templates\own\$template.json"
    }
    elseif ([System.IO.File]::Exists("$scriptPath\templates\default\$template.json")) {
        LoadTemplate "$scriptPath\templates\default\$template.json"
    }
    else {
        Write-Host "The template " -NoNewline
        Write-Host $template -ForegroundColor Red -NoNewline
        Write-Host " does not exist" 
    }
}
else {
    $defaultTemplatePath = "$scriptPath\templates\default";
    $ownTemplatePath = "$scriptPath\templates\own";

    $defaultTemplates = Get-ChildItem $defaultTemplatePath;

    if (Test-Path -Path $ownTemplatePath) {
        $ownTemplates = Get-ChildItem $ownTemplatePath;
    }
    else {
        $ownTemplates = @();
    }

    PrintLogo
    Write-Host
    Write-Host "[$defaultTemplatePath]" -ForegroundColor DarkGray
    Write-Host "------<Default>------"
    foreach ($dTemplate in $defaultTemplates) {
        Write-Host " - " -NoNewline
        Write-Host ($dTemplate -replace ".json", "") -NoNewline

        if ($ownTemplates -match $dTemplate) {
            Write-Host " (overwritten)" -NoNewline -ForegroundColor Green
        }

        Write-Host
    }
    Write-Host "--------------------"
    
    Write-Host
    Write-Host "[$ownTemplatePath]" -ForegroundColor DarkGray
    Write-Host "------<Own>------"
    if ($ownTemplates.Count -ne 0) {
        foreach ($oTemplate in $ownTemplates) {
            Write-Host " - " -NoNewline
            Write-Host ($oTemplate -replace ".json", "")
        }
    }
    else {
        Write-Host " - " -NoNewline
        Write-Host "No own templates configured" -ForegroundColor Red
    }

    Write-Host "-----------------"
    Write-Host
}
