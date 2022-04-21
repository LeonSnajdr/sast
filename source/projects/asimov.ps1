# Include
. ..\libs\QuietusPlus-WriteMenu\Write-Menu.ps1

$menuReturn = Write-Menu -Title 'Sam.Start Asimov' -MultiSelect -Entries @(
    'AgendChat'
    'Admin'
    'Bot'
    'Dispatcher'
    'SocketServer'
)

foreach($service in $menuReturn) {

    Timeout /T 2

    if($service -eq 'AgendChat') {
        wt -w Asimov nt -p "Windows PowerShell" --title $service -d C:\Repos\asimov\Sources\App.AgentChat
        continue
    }

    Write-Host $service
    wt -w Asimov nt -closeOnExit never -p "Windows PowerShell" --title $service dotnet run --project=C:\Repos\asimov\Sources\Asimov.$service
}

# Start-Process wt.exe -ArgumentList "PowerShell.exe", "-NoExit", "-Command", "dotnet run --project=C:\Repos\asimov\Sources\Asimov.Admin"