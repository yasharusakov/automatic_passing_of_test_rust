![Language]( 	https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Language](https://img.shields.io/badge/Node.js-43853D?style=for-the-badge&logo=node.js&logoColor=white)
![Language](https://img.shields.io/badge/TypeScript-007ACC?style=for-the-badge&logo=typescript&logoColor=white)
![Language](https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB)
![Platform](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)
![Platform](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white)
![Browser](https://img.shields.io/badge/Firefox_Browser-FF7139?style=for-the-badge&logo=Firefox-Browser&logoColor=white)
# Project "automatic_passing_of_test_rust"

## Requirements for development

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
