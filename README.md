# sast

Start Applications Seamlessly Timed (sast) is a project that helps you easily launch tasks, organize them into sets, and provides interactive terminals to manage and interact with them. This README will guide you through the setup process for development.

sast uses the following shells to run tasks and interactive terminals:

| Platform | Required shell |
| --- | --- |
| Windows | [PowerShell 7](https://learn.microsoft.com/powershell/scripting/install/installing-powershell-on-windows) (`pwsh.exe`) |
| macOS | Zsh (`zsh`, included with macOS) |
| Linux | Bash (`bash`, available on `PATH`) |

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Setup for Development](#setup-for-development)
3. [Contributing](#contributing)
4. [License](#license)

---

## Prerequisites

Before you begin, ensure you have the following installed:

1. [Rust](https://www.rust-lang.org/learn/get-started)
2. [Tauri system dependencies](https://v2.tauri.app/start/prerequisites/#system-dependencies) for your platform
3. [Mise](https://mise.jdx.dev)

---

## Setup for Development

To get your development environment set up, follow these steps:

1. **Clone the repository**  
   Clone the repository and enter the app directory:

    ```bash
    git clone https://github.com/LeonSnajdr/sast.git
    cd sast/sources/app
    ```

2. **Install dependencies**
   Install the development tools and frontend packages:

    ```bash
    mise install
    yarn install
    ```

3. **Setup backend**
   Create the development database, run migrations, and prepare query metadata:

    ```bash
    cd src-tauri
    cargo sqlx database create
    cargo sqlx migrate run
    cargo sqlx prepare
    cd ..
    ```

4. **Run sast**

    ```bash
    yarn dev
    ```

---

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request. I appreciate all feedback and suggestions to improve the project.

---

## License

This project is licensed under the [Prosperity Public License](LICENSE.md). Feel free to browse and contribute to the codebase under these guidelines.

---

Thank you for using sast! If you have any questions or issues with setup, please feel free to [open an issue](https://github.com/LeonSnajdr/sast/issues) on GitHub. Have fun building and stay secure!
