$scriptPath = split-path -parent $MyInvocation.MyCommand.Definition
$addPath = "$scriptPath\"

$systemPathVariables = $env:Path -split ";";

$alreadyAdded = $false;

foreach ($path in $systemPathVariables) {
    Write-Host $path
    if ($path -eq $addPath) {
        $alreadyAdded = $true;
    }
}

if (-not $alreadyAdded) {
    Write-Host "Added the path: $addPath to the environment variables, please restart your terminal to use sast!"
    [Environment]::SetEnvironmentVariable("Path", $env:Path + ";$addPath", [System.EnvironmentVariableTarget]::User)
}
else {
    Write-Error "Already added to the environment variables"
}
