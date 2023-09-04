![Platform](https://img.shields.io/badge/platform-linux-important?style=flat-square&logo=linux&logoColor=white)
![Code Size](https://img.shields.io/github/languages/code-size/yasharusakov/automatic_passing_of_test_rust)
![License](https://img.shields.io/github/license/yasharusakov/automatic_passing_of_test_rust)

# Project "automatic_passing_of_test_rust"

## Requirements

To successfully compile and run the project, you must have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [System Dependencies](https://tauri.app/v1/guides/getting-started/prerequisites/#setting-up-linux)
- [Node.js](https://nodejs.org/) and npm
- Mozilla Firefox browser
- Geckodriver

## Installation

Install Tauri CLI

```shell
cargo install tauri-cli
```

Clone the repository:

```shell
git clone https://github.com/yasharusakov/automatic_passing_of_test_rust.git
cd automatic_passing_of_test_rust
```

Install interface dependencies:

At the `root` of the project:

```shell
npm install
```

Install the dependencies for the Rust part:

In the `src-tauri` directory:

```shell
cd src-tauri
cargo update
```

## Development and Release

Once the dependencies are installed and configured, you can use the Tauri CLI to manage the development process and create the final release:

At the `root` of the project:

For development, use:

```shell
cargo tauri dev
```

To create a final release, use:

```shell
cargo tauri build
```

## License
This project is licensed under the terms of the [license](LICENSE.txt).
