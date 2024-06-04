# Project Name

This project is built using Rust, a systems programming language focused on safety, speed, and concurrency.

## Installation

To install Rust, follow these steps:

### On Unix-like systems (Linux, macOS)

1. Open your terminal.
2. Run the following command to download and install `rustup`, the Rust toolchain installer:

    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

3. Follow the on-screen instructions to complete the installation.
4. Add the Rust binaries to your PATH by running:

    ```sh
    source $HOME/.cargo/env
    ```

### On Windows

1. Go to [rustup.rs](https://rustup.rs/).
2. Download and run the `rustup-init.exe` installer.
3. Follow the on-screen instructions to complete the installation.
4. Open a new Command Prompt to ensure the environment variables are updated.

### Verify Installation

To verify that Rust is installed correctly, run the following command in your terminal or Command Prompt:

```sh
rustc --version
