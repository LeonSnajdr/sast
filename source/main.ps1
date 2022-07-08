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

    Clear-Host
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
    $projectName = $h.Get_Item("projectName")
    $projectPath = $h.Get_Item("projectPath")
    $services = $h.Get_Item("services") -split ', '
    $apps = $h.Get_Item("apps") -split ', '

    StartProject $ProjectName $projectPath $services $apps
}

if ([string]::IsNullOrWhiteSpace($filePath)) {
    MainMenu
}
else {
    StartWithFile $filePath
}
