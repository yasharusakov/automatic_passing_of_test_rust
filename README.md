![Language](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Language](https://img.shields.io/badge/Node.js-43853D?style=for-the-badge&logo=node.js&logoColor=white)
![Language](https://img.shields.io/badge/TypeScript-007ACC?style=for-the-badge&logo=typescript&logoColor=white)
![Language](https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB)
![Platform](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)
![Platform](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white)
![Browser](https://img.shields.io/badge/Firefox_Browser-FF7139?style=for-the-badge&logo=Firefox-Browser&logoColor=white)
# Project "automatic_passing_of_test_rust"

This project was created in 10th grade to automatically pass tests on the naurok.com.ua website. The goal was to save time and avoid having to manually answer the questions.

The [original version](https://github.com/yasharusakov/automatic_passing_of_test) of the project was written in Node.js, but it had some drawbacks that made it difficult to use. For example, it had no graphical user interface (GUI), so users had to interact with it through the command line. Additionally, users had to install dependencies and configure the project before they could use it.

To address these issues, I rewrote the project in Rust. The new version has a GUI, so users can easily interact with it. Additionally, the project is now self-contained, so users don't need to install any dependencies.

## How to use it
Remember that every action is mandatory.

### Linux
- Install `Firefox browser`.
- Install `geckodriver`. You can do this with your distribution's package manager. If it is not available, you can install it manually from the official [Mozilla releases](https://github.com/mozilla/geckodriver/releases) and add it to `$PATH`.
- Download the package for your Linux distribution from the [releases](https://github.com/yasharusakov/automatic_passing_of_test_rust/releases) of this project.
- Run the application.

### Windows
- Install `Firefox browser`.
- Install [geckodriver.exe](https://github.com/mozilla/geckodriver/releases) from the official Mozilla releases.
- Download the package for your architecture from the [releases](https://github.com/yasharusakov/automatic_passing_of_test_rust/releases) of this project and extract the package containing two files (keep them together).
- Run `geckodriver.exe`.
- Run `automatic-passing-of-test.exe`.

The antivirus might flag this as malware, so disable it for now.

## Requirements for development

To successfully compile and run the project, you must have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [System Dependencies](https://tauri.app/v1/guides/getting-started/prerequisites/#setting-up-linux)
- [Node.js](https://nodejs.org/) and npm
- Mozilla Firefox browser
- Geckodriver
  - Windows: you need to download manually [geckodriver](https://github.com/mozilla/geckodriver/releases) from official mozilla releases
  - Arch Linux: `sudo pacman -S geckodriver`
  - Debian: you need to download manually and add geckogriver to `$PATH`

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
