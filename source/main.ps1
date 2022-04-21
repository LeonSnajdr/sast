# Include
. .\libs\QuietusPlus-WriteMenu\Write-Menu.ps1

$menuReturn = Write-Menu -Title 'Sam.Start Main menu' -Entries @(
    'asimov'
    'ssp'
    'metis'
)

& .\projects\$menuReturn + '.ps1'
