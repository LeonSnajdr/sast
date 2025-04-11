# sast

Start Applications Seamlessly Timed (sast) is a project that helps you easily launch tasks, organize them into sets, and provides interactive terminals to manage and interact with them. This README will guide you through the setup process for development.

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Setup for Development](#setup-for-development)
3. [Contributing](#contributing)
4. [License](#license)

---

## Prerequisites

Before you begin, ensure you have the following installed:

1. [Rust](https://www.rust-lang.org/learn/get-started)
2. [sqlx-cli](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md)
3. [Volta](https://volta.sh/)

---

## Setup for Development

To get your development environment set up, follow these steps:

1. **Clone the repository**  
   Clone the sast repository to your local machine:

    ```bash
    git clone https://github.com/LeonSnajdr/sast.git
    ```

2. **Setup backend**
   Navigate to tauri directory

    ```bash
    cd sources/app/src-tauri
    ```

    Create dev database and execute migrations

    ```bash
    cargo sqlx database create
    ```

    ```bash
    cargo sqlx migrate run
    ```

    ```bash
    cargo sqlx prepare
    ```

3. **Setup frontend**  
   Navigate to frontend directory

    ```bash
    cd sources/app
    ```

    Install node packages

    ```bash
    yarn install
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

This project is licensed under the [MIT License](LICENSE). Feel free to browse and contribute to the codebase under these guidelines.

---

Thank you for using sast! If you have any questions or issues with setup, please feel free to [open an issue](https://github.com/LeonSnajdr/sast/issues) on GitHub. Have fun building and stay secure!
