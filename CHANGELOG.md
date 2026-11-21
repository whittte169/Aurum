# Changelog

All notable changes to **Aurum Terminal** are documented in this file.

## [1.0.0-alpha.2] — 2026

### Added

* **Asynchronous Task Execution:** Integrated the `tokio` runtime and `std::sync::mpsc` channels. PowerShell commands and system information gathering (`aurum -system`) now execute in a background thread, preventing UI freezes (`main.rs`, `ui.rs`, `network.rs`, `system.rs`).
* **License Notice on Startup:** Added an `MIT License` line to the terminal welcome banner (`ui.rs`).

### Changed

* **Command Prompt Format:** Changed the prompt separator in both the input line and output logs from `>` to `$` (e.g., `C:\Users\User $ `) (`ui.rs`).
* **Custom Title Bar Button Rendering:** Replaced the standard `egui::Button` widget in `title_button` with custom rendering via `ui.painter()` for finer control over borders, corner rounding, and hover states (`ui.rs`).
* **Window Control Icons:** Updated the maximize/restore and close button symbols to `🗗`, `🗖`, and `✕` (previously `❐`, `□`, and `×`) (`ui.rs`).
* **Internal Application Name:** Changed the application name parameter in `eframe::run_native` from `"Aurum Terminal"` to `"Aurum"` (`main.rs`).

### Fixed

* **Help Text Typo:** Fixed a typo in the hint message displayed when running the base `aurum` command (`aurum-version` corrected to `aurum -version`) (`ui.rs`).

## [1.0.0-alpha.1] — 2026

### Added

* Initial alpha release of **Aurum Terminal**.
* Custom frameless window featuring a golden visual theme and custom window controls (`ui.rs`).
* Seamless background PowerShell integration without spawning external console windows (`network.rs`).
* Built-in commands: `aurum`, `aurum -version`, `aurum -system`, `cls`, `clear`, and `exit`.
* Hardware and OS diagnostics module covering CPU, GPU, RAM (including WMI queries), and storage drives (`system.rs`).
* Command history navigation using the `Up` and `Down` arrow keys.