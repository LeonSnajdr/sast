# Sam.Start

Start projects with a selection or with start templates, which you can also create yourself.

# Configuration

## Grant permission

You have to give permissions to sast.ps1 and Write-Menu.ps1 to disable annoying security warnings

## Configure the script

| Option            | Description                                                                                  |
| ----------------- | -------------------------------------------------------------------------------------------- |
| `reposPath`       | Path of the repos folder                                                                     |
| `compileSolution` | Determine if you want to compile the whole solution (recommended for multiple services)      |
| `compileProject`  | Determine if you want to compile the individual services (not recommended for many services) |

# UI

Conveniently start your projects with a user interface

## Usage

- Select the script and run with powershell
- Select the project you want to start
- Select all services you want to start
- Press enter to start them

# Comandline

Start your projects with the comandline and use templates

## Usage

.\sast.ps1

| Parameter | Description                        |
| --------- | ---------------------------------- |
| `-f`      | Path of template you want to start |

# Templates

Create templates specific to your needs to start projects

> To separate projects from each other ': ' is used, to separate services from each other ', ' is used.

| Parameter            | Description                                                                                                                                                                      |
| -------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `projectNames`       | The names of the projects you want to start. If two projects should be started in one terminal, the same name must be used here, since this also becomes the id of the terminal. |
| `projectPaths`       | Starting from the repos folder this is the path to the service folder                                                                                                            |
| `servicesForProject` | These are all services that will be started                                                                                                                                      |
| `appsForProject`     | These are the services that are started by yarn, in most cases the forntends                                                                                                     |
