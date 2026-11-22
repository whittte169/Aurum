#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod network;
mod system;
mod ui;

use eframe::egui;
use std::sync::Arc;
use tokio;

fn main() -> eframe::Result<()> {
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio Runtime");
    let _guard = rt.enter();

    tokio::spawn(async {
        
    });

    let icon = image::load_from_memory(
        include_bytes!("../assets/icons/terminal.png"),
    )
    .expect("Failed to load application icon")
    .to_rgba8();

    let (width, height) = icon.dimensions();
    let icon_data = egui::IconData {
        rgba: icon.into_raw(),
        width,
        height,
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1050.0, 700.0])
            .with_min_inner_size([760.0, 520.0])
            .with_title("Aurum Terminal")
            .with_decorations(false)
            .with_resizable(true)
            .with_icon(Arc::new(icon_data)),
        ..Default::default()
    };

    eframe::run_native(
        "Aurum",
        options,
        Box::new(|cc| Ok(Box::new(ui::AurumApp::new(cc)))),
    )
}
