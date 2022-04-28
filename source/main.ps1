# Include
. .\libs\QuietusPlus-WriteMenu\Write-Menu.ps1

Get-Content "./config.txt" | foreach-object -begin {$h=@{}} -process { $k = [regex]::split($_,'='); if(($k[0].CompareTo("") -ne 0) -and ($k[0].StartsWith("[") -ne $True)) { $h.Add($k[0], $k[1]) } }
$ReposPath = $h.Get_Item("ReposPath")

function MainMenu {
    $projectName = Write-Menu -Title "Sam.Start (c)lsnajdr" -Entries @(
        'Asimov'
        'SelfServicePortal'
        'Metis'
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

            foreach ($service in $menuReturn) {

                Write-Output "Starting $service..."
                Timeout /T 2

                if ($service -eq 'AgentChat') {
                    wt -w Asimov nt -p "Windows PowerShell" --title "$service" -d "$ReposPath\asimov\Sources\App.$service" cmd /k "yarn install && yarn serve" 
                    continue
                }

                Write-Host $service
                wt -w Asimov nt -p "Windows PowerShell" --title "$service" -d "$ReposPath\asimov\Sources\Asimov.$service" cmd /k "dotnet run" 
            }
        }
        Default {
            Write-Host "No matching project selected"
            MainMenu
        }
    }
    
}

MainMenu
