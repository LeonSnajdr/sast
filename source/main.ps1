Param(
    [parameter(Mandatory = $false)]
    [alias("f")]
    [string] $filePath
)

# Include
. .\libs\QuietusPlus-WriteMenu\Write-Menu.ps1

Get-Content "./config.txt" | foreach-object -begin { $h = @{} } -process { $k = [regex]::split($_, '='); if (($k[0].CompareTo("") -ne 0) -and ($k[0].StartsWith("[") -ne $True)) { $h.Add($k[0], $k[1]) } }
$ReposPath = $h.Get_Item("ReposPath")
$CompileSolution = [System.Convert]::ToBoolean($h.Get_Item("CompileSolution"))
$CompileProject = [System.Convert]::ToBoolean($h.Get_Item("CompileProject"))

function PrintLogo {
    Clear-Host
    Write-Host "   _____                  _____ __             __ "
    Write-Host "  / ___/____ _____ ___   / ___// /_____ ______/ /_"
    Write-Host "  \__ \/ __ '/ __ '__ \  \__ \/ __/ __ '/ ___/ __/"
    Write-Host " ___/ / /_/ / / / / / / ___/ / /_/ /_/ / /  / /_  "
    Write-Host "/____/\__,_/_/ /_/ /_(_)____/\__/\__,_/_/   \__/  "
    Write-Host "                                                  "
    Write-Host "(c)lsnajdr"
    Write-Host ""
}

function MainMenu {
    $projectName = Write-Menu -Title "Sam.Start (c)lsnajdr" -Entries @(
        'Asimov'
        'SelfServicePortal'
        'Metis'
        'Configuration'
    )

    DrawProjectMenu $projectName
}

function DrawProjectMenu($project) {

    switch ($project) {
        "Asimov" { 
            $services = Write-Menu -Title "($ReposPath/$project) [INSERT] Select everything [SPACE] Select / Deselect [ENTER] Start selected projects" -MultiSelect -Entries @(
                'App.AgentChat'
                'Asimov.Admin'
                'Asimov.Bot'
                'Asimov.Dispatcher'
                'Asimov.SocketServer'
                'Asimov.TaskScheduler'
            )

            StartProject 'Asimov' 'asimov\Sources' $services @('App.AgentChat')

            Read-Host "Press [ENTER] to get back to the main menu"
        }

        "Configuration" {
            Write-Host "Configuration"
            Write-Host "Repos path: $ReposPath"
            Write-Host "CompileSolution: $CompileSolution"
            Write-Host "CompileProject: $CompileProject"
            Read-Host "Press [ENTER] to get back to the main menu"
            MainMenu
        }

        Default {
            Write-Host "No matching project selected"
            Read-Host "Press [ENTER] to get back to the main menu"
            MainMenu
        }
    }
    
}

function StartProject ($projectName, $projectPath, $services, $apps) {

    PrintLogo
    Write-Host $projectPath

    if ($CompileSolution) {
        Write-Host "Compiling solution..."
        dotnet build "$ReposPath\$projectPath" | Out-Null
    }

    foreach ($service in $services) {

        $percent = [Math]::Round((($services.IndexOf($service)) / $services.Count) * 100, 0)

        $status = "Starting $service..."
        if ($CompileProject) {
            $status = "Compiling & Starting $service..."
        }

        Write-Progress -Activity "Starting selected Projects" -Status $status -PercentComplete  $percent

        if ($apps -contains $service) {
            wt -w $projectName nt -p "Windows PowerShell" --title "$service" -d "$ReposPath\$projectPath\$service" cmd /k "yarn install && yarn serve" 
        }
        else {
            if ($CompileProject) {
                dotnet build "$ReposPath\$projectPath\$service" | Out-Null
            }

            wt -w $projectName nt -p "Windows PowerShell" --title "$service" -d "$ReposPath\$projectPath\$service" cmd /k "dotnet run"
        }

        if ($services.IndexOf($service) -eq 0 -and !$CompileProject) {
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


        StartProject $ProjectName $projectPath $services $apps
    }

    Write-Host "All projects and servies successfully started, I wish a pleasant work"
}

if ([string]::IsNullOrWhiteSpace($filePath)) {
    MainMenu
}
else {
    StartWithFile $filePath
}
