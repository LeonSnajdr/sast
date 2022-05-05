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

    DrawProjectMenu ($projectName)
}

function DrawProjectMenu($project) {

    switch ($project) {
        "Asimov" { 
            $menuReturn = Write-Menu -Title "($ReposPath/$project) [INSERT] Select everything [SPACE] Select / Deselect [ENTER] Start selected projects" -MultiSelect -Entries @(
                'AgentChat'
                'Admin'
                'Bot'
                'Dispatcher'
                'SocketServer'
            )

            if ($CompileSolution) {
                Write-Host "Compiling solution..."
                dotnet build "$ReposPath\asimov\Sources" | Out-Null
            }

            foreach ($service in $menuReturn) {

                $percent = [Math]::Round((($menuReturn.IndexOf($service)) / $menuReturn.Count) * 100, 0)

                $status = "Starting $service..."
                if ($CompileProject) {
                    $status = "Compiling & Starting $service..."
                }

                Write-Progress -Activity "Starting selected Projects" -Status $status -PercentComplete  $percent

                if ($service -eq 'AgentChat') {
                    wt -w Asimov nt -p "Windows PowerShell" --title "$service" -d "$ReposPath\asimov\Sources\App.$service" cmd /k "yarn install && yarn serve" 
                }
                else {
                    if ($CompileProject) {
                        dotnet build "$ReposPath\asimov\Sources\Asimov.$service" | Out-Null
                    }
    
                    wt -w Asimov nt -p "Windows PowerShell" --title "$service" -d "$ReposPath\asimov\Sources\Asimov.$service" cmd /k "dotnet run"
                }

                if ($menuReturn.IndexOf($service) -eq 0 -and !$CompileProject) {
                    # Wait two seconds before opening the next projects to open in same terminal window 
                    Timeout /T 2 | Out-Null
                }
            }

            Read-Host "Press [ENTER] to get back to the main menu"
            MainMenu
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

MainMenu
