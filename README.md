# Aurum Terminal

**A modern, lightweight Windows terminal built with Rust.**

Aurum Terminal is a custom Windows terminal application designed with simplicity, performance, and a distinctive golden interface in mind. Built using Rust and egui, it combines a minimal command-line experience with convenient system information commands and a custom window interface.

<p align="center">
  <img src="assets/icons/terminal.png" width="96" alt="Aurum Terminal Icon">
</p>

<p align="center">
  <strong>Minimal design. Native commands. Rust performance.</strong>
</p>

---

## Overview

Aurum Terminal provides a clean environment for executing PowerShell commands without opening a separate PowerShell window.

The application features a custom frameless window, a golden visual theme, command history, and built-in commands for inspecting application and system information.

## Features

- **Custom Interface** — A modern golden-themed interface with a custom title bar and window controls.
- **PowerShell Integration** — Execute PowerShell commands directly inside Aurum Terminal.
- **System Information** — Retrieve information about your operating system, CPU, GPU, RAM, and storage devices.
- **Command History** — Navigate previously executed commands using the Up and Down arrow keys.
- **Built-in Commands** — Access application version and system information directly.
- **Lightweight** — Developed in Rust with egui.
- **No Separate PowerShell Window** — Commands execute in the background without opening another console window.

## Built-in Commands

| Command | Description |
|---|---|
| `aurum -version` | Displays the current application version. |
| `aurum -system` | Displays system hardware and operating system information. |
| `cls` | Clears the terminal output. |
| `clear` | Clears the terminal output. |
| `exit` | Displays the application exit instruction. |

All other commands are passed to PowerShell.

## System Information

The `aurum -system` command provides information about:

| Component | Information |
|---|---|
| Operating System | Windows version |
| CPU | Processor model and usage |
| GPU | Graphics processor model and usage |
| RAM | Total and used memory |
| Memory Modules | Manufacturer, model, capacity, and speed |
| Storage | Available drives, used space, and capacity |

The availability of certain hardware details depends on Windows and the underlying system interfaces.

## Requirements

- Windows 10 or Windows 11
- PowerShell
- Rust toolchain (only required for building from source)

## Installation

### Download

Download the latest available release from the GitHub Releases page:

[**Aurum Terminal Releases**](https://github.com/whittte169/Aurum/releases)

Extract the downloaded archive, if applicable, and run `aurum-terminal.exe`.

### Build from Source

Clone the repository:

```bash
git clone https://github.com/whittte169/Aurum.git
cd Aurum
```

Build the application in release mode:

```bash
cargo build --release
```

The compiled executable will be available at:

```text
target/release/aurum-terminal.exe
```

## Project Structure

```text
Aurum/
├── assets/
│   └── icons/
│       └── terminal.png
├── src/
│   ├── main.rs
│   ├── ui.rs
│   ├── network.rs
│   └── system.rs
├── Cargo.toml
├── LICENSE
└── README.md
```

| File | Purpose |
|---|---|
| `main.rs` | Application entry point and window configuration. |
| `ui.rs` | Terminal interface, rendering, and command handling. |
| `network.rs` | PowerShell execution and GitHub release checks. |
| `system.rs` | Hardware and operating system information. |

## Technologies

Aurum Terminal is built using:

- **Rust** — Core application language.
- **egui / eframe** — Graphical user interface.
- **PowerShell** — Command execution.
- **sysinfo** — System and hardware information.
- **WMI** — Windows hardware information.
- **gfxinfo** — GPU information.
- **reqwest** — GitHub release information and update checks.

## Development Status

Aurum Terminal is currently under active development.

Some features and internal implementations may change between releases. Compatibility and functionality improvements are expected as the project evolves.

## License

This project is distributed under the license specified in the [`LICENSE`](LICENSE) file.

## Repository

**GitHub:** https://github.com/whittte169/Aurum

---

<p align="center">
  <strong>Aurum Terminal</strong><br>
  Copyright (c) whittte169 2026
</p>