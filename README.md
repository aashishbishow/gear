# Gear - Frontend Project Setup Tool

## Table of Contents
- [Introduction](#introduction)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Configuration](#configuration)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

## Introduction
Gear is a command-line tool designed to streamline the setup of frontend projects using Vite, React, and TailwindCSS. It helps developers quickly scaffold new projects with a consistent structure and pre-configured dependencies, saving time and effort.

## Features
- Initialize new frontend projects with Vite and React
- Support for both JavaScript and TypeScript
- Automatic installation of dependencies
- Pre-configured TailwindCSS setup
- Progress bar to track setup steps

## Installation
To install Gear, clone the repository and build the project using Cargo:
1. Clone the repository:
    ```sh
    git clone https://github.com/aashishbishow/gear.git
    ```
2. Navigate to the project directory:
    ```sh
    cd gear
    ```
3. Build the project:
    ```sh
    cargo build --release
    ```
4. Add the binary to your PATH (optional):
    ```sh
    export PATH=$PATH:/path/to/gear/target/release
    ```

## Usage
To create a new frontend project, use the following command:
```sh
cargo run -- --name myproject --lang js init
```
Replace `myproject` with your desired project name and `js` with `ts` if you prefer TypeScript.

### Example
```sh
cargo run -- --name awesome-app --lang ts init
```

## Configuration
Gear automatically configures your project with the necessary files and settings. However, you can customize the configuration by editing the following files:
- `vite.config.js` or `vite.config.ts`: Vite configuration file
- `src/index.css`: TailwindCSS configuration

## Contributing
We welcome contributions to Gear! To contribute, follow these steps:
1. Fork the repository
2. Create a new branch: `git checkout -b feature-branch`
3. Make your changes and commit them: `git commit -m 'Add new feature'`
4. Push to the branch: `git push origin feature-branch`
5. Open a pull request

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact
For questions or feedback, please reach out via email at [aashishbishowkarma@outlook.com](mailto:aashishbishowkarma@outlook.com).
