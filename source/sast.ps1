Param(
    [parameter(Mandatory = $false)]
    [alias("t")]
    [string] $template,
    [parameter(Mandatory = $false)]
    [alias("f")]
    [string] $file
)

$scriptPath = split-path -parent $MyInvocation.MyCommand.Definition

Get-Content "$scriptPath\config.txt" | foreach-object -begin { $h = @{} } -process { $k = [regex]::split($_, '='); if (($k[0].CompareTo("") -ne 0) -and ($k[0].StartsWith("[") -ne $True)) { $h.Add($k[0], $k[1]) } }
$reposPath = $h.Get_Item("reposPath")
$compileSolution = [System.Convert]::ToBoolean($h.Get_Item("compileSolution"))
$compileProject = [System.Convert]::ToBoolean($h.Get_Item("compileProject"))

function PrintLogo {
    Clear-Host 
    Write-Host "   _____                  _____ __             __ " 
    Write-Host "  / ___/____ _____ ___   / ___// /_____ ______/ /_"
    Write-Host "  \__ \/ __ '/ __ '__ \  \__ \/ __/ __ '/ ___/ __/"
    Write-Host " ___/ / /_/ / / / / / / ___/ / /_/ /_/ / /  / /_  "
    Write-Host "/____/\__,_/_/ /_/ /_(_)____/\__/\__,_/_/   \__/  "
    Write-Host "                                                  "
    Write-Host "Author      : " -NoNewline
    Write-Host "Leon Snajdr" -ForegroundColor DarkCyan
    Write-Host "Version     : v0.1.2"
}


function StartProject ($projectName, $projectPath, $services, $apps) {

    PrintLogo
    Write-Host "Template    : $template"
    Write-Host "Working dir : $projectPath"
    Write-Host

    foreach ($app in $apps) {
        if ([string]::IsNullOrEmpty($app)) {
            continue;
        }

        Write-Host "Starting app $app" -ForegroundColor DarkGray

        wt -w $projectName nt -p "Windows PowerShell" --title "$app" -d "$reposPath\$projectPath\$app" powershell -noExit "yarn serve" 
        
        Start-Sleep -Seconds 2
    }

    if ($compileSolution) {
        Write-Host "Compiling solution..." -ForegroundColor DarkGray
        dotnet build "$reposPath\$projectPath" | Out-Null
    }

    foreach ($service in $services) {
        $status = "Starting $service..."
        if ($compileProject) {
            $status = "Compiling & Starting $service..."
        }

        Write-Host $status -ForegroundColor DarkGray

        if ($compileProject) {
            dotnet build "$reposPath\$projectPath\$service" | Out-Null
        }

        wt -w $projectName nt -p "Windows PowerShell" --title "$service" -d "$reposPath\$projectPath\$service" powershell -noExit "dotnet run --no-build"

        Start-Sleep -Seconds 0.5

        if ($services.IndexOf($service) -eq 0 -and !$compileProject) {
            # Wait two seconds before opening the next projects to open in same terminal window 
            Start-Sleep -Seconds 1.5
        }
    }
}
function StartWithFile($filePath) {

    Get-Content $filePath | foreach-object -begin { $h = @{} } -process { $k = [regex]::split($_, '='); if (($k[0].CompareTo("") -ne 0) -and ($k[0].StartsWith("[") -ne $True)) { $h.Add($k[0], $k[1]) } }
    $projectNames = $h.Get_Item("projectNames") -split ": "
    $projectPaths = $h.Get_Item("projectPaths") -split ": "
    $servicesForProject = $h.Get_Item("servicesForProject") -split ': '
    $appsForProject = $h.Get_Item("appsForProject") -split ': '
    
    for ($i = 0; $i -lt $projectNames.Count; $i++) {
        $projectName = $projectNames[$i]
        $projectPath = $projectPaths[$i]
        $services = $servicesForProject[$i] -split ', '
        $apps = $appsForProject[$i] -split ', '


        StartProject $projectName $projectPath $services $apps
    }

    Write-Host
    Write-Host "All projects and services started successfully, I wish a pleasant work" -ForegroundColor DarkGreen
    Write-Host
}


if (-not [string]::IsNullOrWhiteSpace($template)) {
    if ([System.IO.File]::Exists("$scriptPath\templates\own\$template.txt")) {
        StartWithFile "$scriptPath\templates\own\$template.txt"
    }
    elseif ([System.IO.File]::Exists("$scriptPath\templates\default\$template.txt")) {
        StartWithFile "$scriptPath\templates\default\$template.txt"
    }
    else {
        Write-Error "This template does not exist!"
    }
}
elseif (-not [string]::IsNullOrWhiteSpace($file)) {
    StartWithFile $file
}
else {
    $defaultTemplatePath = "$scriptPath\templates\default";
    $ownTemplatePath = "$scriptPath\templates\own";

    PrintLogo
    Write-Host
    Write-Host "[$defaultTemplatePath]" -ForegroundColor DarkGray
    Write-Host "------<Default>------"
    foreach ($dTemplate in Get-ChildItem $defaultTemplatePath) {
        Write-Host " - " -NoNewline
        Write-Host ($dTemplate -replace ".txt", "")
    }
    Write-Host "--------------------"
    
    Write-Host
    Write-Host "[$ownTemplatePath]" -ForegroundColor DarkGray
    Write-Host "------<Own>------"
    if (Test-Path -Path $ownTemplatePath) {
        foreach ($oTemplate in Get-ChildItem $ownTemplatePath) {
            Write-Host " - " -NoNewline
            Write-Host ($oTemplate -replace ".txt", "")
        }
    }
    else {
        Write-Host " - " -NoNewline
        Write-Host "No own templates configured" -ForegroundColor Red
    }

    Write-Host "-----------------"
    Write-Host
}
