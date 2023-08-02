Param(
    [parameter(Mandatory = $true)]
    [string] $repoDirecotry,
    [parameter(Mandatory = $true)]
    [string] $sourceBranch
)

Write-Host
Push-Location $repoDirecotry

git checkout $sourceBranch
git pull

Write-Host "Enter the name of the branch you want to create: "
$branchName = Read-Host

git checkout -b $branchName

Pop-Location
Write-Host