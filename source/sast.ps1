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
    Write-Host "(c)lsnajdr v0.0.1"
    Write-Host ""
}


function StartProject ($projectName, $projectPath, $services, $apps) {

    PrintLogo
    Write-Host $projectPath

    if ($compileSolution) {
        Write-Host "Compiling solution..."
        dotnet build "$reposPath\$projectPath" | Out-Null
    }

    foreach ($service in $services) {
        $status = "Starting $service..."
        if ($compileProject) {
            $status = "Compiling & Starting $service..."
        }

        Write-Host $status

        if ($apps -contains $service) {
            wt -w $projectName nt -p "Windows PowerShell" --title "$service" -d "$reposPath\$projectPath\$service" powershell -noExit "yarn serve" 
        }
        else {
            if ($compileProject) {
                dotnet build "$reposPath\$projectPath\$service" | Out-Null
            }

            wt -w $projectName nt -p "Windows PowerShell" --title "$service" -d "$reposPath\$projectPath\$service" powershell -noExit "dotnet run --no-build"
        }

        if ($services.IndexOf($service) -eq 0 -and !$compileProject) {
            # Wait two seconds before opening the next projects to open in same terminal window 
            Timeout /T 2 | Out-Null
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

    Write-Host "All projects and servies successfully started, I wish a pleasant work"
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
    Write-Host
    Write-Host "Please select a start template"
    Write-Host "------<Default>------"
    foreach ($dTemplate in Get-ChildItem "$scriptPath\templates\default") {
        $dTemplate -replace ".txt", ""
    }
    
    Write-Host "------<Own>------"
    $ownTemplatePath = "$scriptPath\templates\own";

    if (Test-Path -Path $ownTemplatePath) {
        foreach ($oTemplate in Get-ChildItem $ownTemplatePath) {
            $oTemplate -replace ".txt", ""
        }
    }
    else {
        Write-Host "No own templates configured"
    }

    Write-Host
}
