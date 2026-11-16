# ConPTY

`conpty.dll` and `OpenConsole.exe` are shipped next to the app executable on Windows (see `tauri.windows.conf.json`).
`portable-pty` prefers a `conpty.dll` found there over the ConPTY that is built into Windows.

The ConPTY built into Windows keeps its own copy of the screen and gets out of sync with xterm.js when the
column count changes, which duplicates or drops lines. The version shipped here passes the shell output through
unchanged and asks the terminal for the cursor position after a resize, so the columns can follow the window size.

- Source: NuGet package `Microsoft.Windows.Console.ConPTY` (https://github.com/microsoft/terminal)
- Version: 1.25.261109001, x64
- License: MIT, © Microsoft Corporation

## Updating

1. Download the package from https://www.nuget.org/packages/Microsoft.Windows.Console.ConPTY (a `.nupkg` is a zip file).
2. Replace `conpty.dll` with `runtimes/win-x64/native/conpty.dll`.
3. Replace `OpenConsole.exe` with `build/native/runtimes/x64/OpenConsole.exe`.

Both files have to come from the same package version.
