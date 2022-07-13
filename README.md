# Sam.Start

Start projects with a selection or with start templates, which you can also create yourself.

# Configuration

## Grant permission

You have to give permissions to sast.ps1 to disable annoying security warnings

## Setup

You can execute the ./setup.ps1 to register the script globally, then it gets globally avaliable with `sast`

## Configure the script

| Option            | Description                                                                                  |
| ----------------- | -------------------------------------------------------------------------------------------- |
| `reposPath`       | Path of the repos folder                                                                     |
| `compileSolution` | Determine if you want to compile the whole solution (recommended for multiple services)      |
| `compileProject`  | Determine if you want to compile the individual services (not recommended for many services) |

# Comandline

Start your projects with the comandline and use templates

## Usage

| Parameter | Description                        |
| --------- | ---------------------------------- |
| `-f`      | Path of template you want to start |

The first parameter after the script is interpreted as a template, so -f is not necessarily expected. The templates in the default and own folders are recognised.

> Start the ControlCenter

```shell
sast cc
```

# Templates

Create templates specific to your needs to start projects

> To separate projects from each other ': ' is used, to separate services from each other ', ' is used.

| Parameter            | Description                                                                                                                                                                      |
| -------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `projectNames`       | The names of the projects you want to start. If two projects should be started in one terminal, the same name must be used here, since this also becomes the id of the terminal. |
| `projectPaths`       | Starting from the repos folder this is the path to the service folder                                                                                                            |
| `servicesForProject` | These are all services that will be started                                                                                                                                      |
| `appsForProject`     | These are the services that are started by yarn, in most cases the forntends                                                                                                     |
