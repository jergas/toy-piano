# Development Guide

This guide covers how to set up your development environment to contribute to the **Toy Piano** project.

## Prerequisites

### 1. Rust Toolchain
We use the latest stable Rust. Install it via `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Git
Ensure you have Git installed to manage source control.

## Platform-Specific Setup

### macOS
To align with our open-source philosophy, we prefer using the upstream LLVM toolchain over Apple's proprietary fork.

1.  **macOS SDK**: We must install the Command Line Tools to get the system headers (CoreAudio, Cocoa), as these are proprietary and have no FOSS alternative.
    ```bash
    xcode-select --install
    ```

2.  **Open Source Toolchain**: Install standard LLVM (Clang + LLD) via Homebrew.
    ```bash
    brew install llvm
    ```

3.  **Configuration**: Configure your shell to use the open-source compiler for dependencies.
    Add to your `~/.zshrc`:
    ```bash
    export PATH="/opt/homebrew/opt/llvm/bin:$PATH"
    export CC="/opt/homebrew/opt/llvm/bin/clang"
    export AR="/opt/homebrew/opt/llvm/bin/llvm-ar"
    ```

### Linux (Ubuntu/Debian)
You need development headers for ALSA (audio), system libraries for the GUI (Iced), and build utilities.

```bash
sudo apt-get update
sudo apt-get install build-essential pkg-config \
    libasound2-dev \
    libudev-dev \
    libfreetype6-dev \
    libexpat1-dev \
    libfontconfig1-dev
```

*Note: If you are on a different distro, look for the equivalent packages for ALSA, FreeType, and FontConfig.*

### Windows
1.  **Visual Studio Build Tools**: Install the "Desktop development with C++" workload. This provides the MSVC linker and Windows SDK required by Rust.

## Getting Started

1.  **Clone the Repository**:
    ```bash
    git clone https://github.com/jergas/toy-piano.git
    cd toy-piano
    ```

2.  **Run the Application**:
    ```bash
    cargo run
    ```
    *This will download dependencies, compile the project, and launch the app.*

3.  **Run Tests**:
    ```bash
    cargo test
    ```

## Cross-Compilation (Optional)

If you want to build for other platforms from your local machine:

### To Windows (from macOS/Linux)
We use `cargo-xwin` to compile for Windows using the MSVC target.
```bash
cargo install cargo-xwin
cargo xwin build --release --target x86_64-pc-windows-msvc
```

### To Linux (from macOS)
We use `cross` to build inside a Docker container.
```bash
cargo install cross
cross build --release --target x86_64-unknown-linux-gnu
```

## Project Structure
*   `src/main.rs`: Entry point.
*   `src/audio/`: Audio processing logic (cpal + rustysynth).
*   `src/midi/`: MIDI input handling (midir).
*   `src/ui/`: GUI implementation (iced).
