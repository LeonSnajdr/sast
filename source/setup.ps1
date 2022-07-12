$scriptPath = split-path -parent $MyInvocation.MyCommand.Definition
$env:PATH += "$scriptPath\"