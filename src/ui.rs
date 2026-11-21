
use eframe::egui;
use std::path::PathBuf;
use std::time::Duration;

const VERSION: &str = "1.0.0-alpha.1";
const REPOSITORY: &str = "https://github.com/whittte169/Aurum";

const BACKGROUND: egui::Color32 = egui::Color32::from_rgb(235, 197, 83);
const TITLE_BACKGROUND: egui::Color32 = egui::Color32::from_rgb(194, 145, 30);
const TEXT_COLOR: egui::Color32 = egui::Color32::from_rgb(15, 12, 5);
const HOVER_COLOR: egui::Color32 = egui::Color32::from_rgb(220, 174, 55);
const BORDER_COLOR: egui::Color32 = egui::Color32::from_rgb(112, 76, 12);

pub struct AurumApp {
    command: String,
    output: String,
    current_dir: PathBuf,
    history: Vec<String>,
    history_index: Option<usize>,
    icon: egui::TextureHandle,
}

impl AurumApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let current_dir = std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("C:\\"));

        let icon_image = image::load_from_memory(
            include_bytes!("../assets/icons/terminal.png"),
        )
        .expect("Failed to load terminal icon")
        .to_rgba8();

        let (width, height) = icon_image.dimensions();

        let icon = cc.egui_ctx.load_texture(
            "terminal_icon",
            egui::ColorImage::from_rgba_unmultiplied(
                [width as usize, height as usize],
                icon_image.as_raw(),
            ),
            egui::TextureOptions::LINEAR,
        );

        let mut app = Self {
            command: String::new(),
            output: String::new(),
            current_dir,
            history: Vec::new(),
            history_index: None,
            icon,
        };

        app.output.push_str(&format!(
            "Copyright (c) whittte169 2026\n{}\n\n",
            REPOSITORY
        ));

        app
    }

    fn execute_command(&mut self) {
        let command = self.command.trim().to_string();

        if command.is_empty() {
            return;
        }

        self.output.push_str(&format!(
            "{}>{}\n",
            self.current_dir.display(),
            command
        ));

        self.history.push(command.clone());
        self.history_index = None;

        let normalized = command.to_lowercase();

        if normalized.trim() == "aurum" {
            self.output.push_str(&"\nUse aurum -system or aurum-version\n\n");
        } else if normalized == "aurum -version" {
            self.output.push_str(&format!(
                "\nAurum {}\n\n",
                VERSION
            ));
        } else if normalized == "aurum -system" {
            self.output.push_str("\nSystem Information\n");
            self.output.push_str("========================\n\n");

            self.output.push_str(&format!(
                "Operating System: {}\n",
                crate::system::get_os_name()
            ));

            self.output.push_str(&format!(
                "CPU: {}\n",
                crate::system::get_cpu_model()
            ));

            self.output.push_str(&format!(
                "CPU Usage: {:.1}%\n",
                crate::system::get_cpu_usage()
            ));

            self.output.push_str(&format!(
                "GPU: {}\n",
                crate::system::get_gpu_model()
            ));

            self.output.push_str(&format!(
                "GPU Usage: {}%\n",
                crate::system::get_gpu_usage()
            ));

            self.output.push_str(&format!(
                "RAM Usage: {} / {}\n",
                crate::system::get_ram_use(),
                crate::system::get_total_ram()
            ));

            self.output.push_str("\nMemory Modules:\n");

            for ram in crate::system::get_ram_model() {
                self.output.push_str(&format!("  {}\n", ram));
            }

            self.output.push_str("\nDrives:\n");

            for disk in crate::system::get_disks_info() {
                self.output.push_str(&format!("  {}\n", disk));
            }

            self.output.push('\n');
        } else if normalized == "cls" || normalized == "clear" {
            self.output.clear();
        } else if normalized == "exit" {
            self.output.push_str(
                "Use the close button to exit Aurum.\n\n",
            );
        } else {
            match crate::network::execute_command(
                &command,
                &self.current_dir,
            ) {
                Ok(result) => {
                    if !result.trim().is_empty() {
                        self.output.push_str(&result);

                        if !result.ends_with('\n') {
                            self.output.push('\n');
                        }
                    }

                    if normalized == "cd"
                        || normalized.starts_with("cd ")
                        || normalized == "set-location"
                        || normalized.starts_with("set-location ")
                    {
                        let path = command
                            .split_once(' ')
                            .map(|(_, path)| {
                                path.trim().trim_matches('"')
                            })
                            .unwrap_or("");

                        let next = if path.is_empty() {
                            PathBuf::from(
                                std::env::var("USERPROFILE")
                                    .unwrap_or_else(|_| "C:\\".to_string()),
                            )
                        } else if PathBuf::from(path).is_absolute() {
                            PathBuf::from(path)
                        } else {
                            self.current_dir.join(path)
                        };

                        if next.is_dir() {
                            self.current_dir = next;
                        }
                    }
                }
                Err(error) => {
                    self.output.push_str(&format!(
                        "Error: {}\n",
                        error
                    ));
                }
            }

            self.output.push('\n');
        }

        self.command.clear();
    }

    fn navigate_history(&mut self, direction: i32) {
        if self.history.is_empty() {
            return;
        }

        let index = self.history_index.unwrap_or(self.history.len());

        let next = if direction < 0 {
            index.saturating_sub(1)
        } else {
            (index + 1).min(self.history.len())
        };

        self.history_index = Some(next);

        self.command = if next < self.history.len() {
            self.history[next].clone()
        } else {
            String::new()
        };
    }

    fn title_button(
        ui: &mut egui::Ui,
        symbol: &str,
        color: egui::Color32,
        hover: egui::Color32,
    ) -> bool {
        let button = egui::Button::new(
            egui::RichText::new(symbol)
                .size(17.0)
                .color(color),
        )
        .fill(TITLE_BACKGROUND)
        .stroke(egui::Stroke::new(1.0, BORDER_COLOR))
        .corner_radius(egui::CornerRadius::same(5));

        let response = ui.add_sized([38.0, 28.0], button);

        if response.hovered() {
            ui.painter().rect_filled(
                response.rect,
                5.0,
                hover,
            );

            ui.painter().rect_stroke(
                response.rect,
                5.0,
                egui::Stroke::new(1.0, BORDER_COLOR),
                egui::StrokeKind::Inside,
            );
        }

        response.clicked()
    }

    fn title_bar(&mut self, ui: &mut egui::Ui) {
        let width = ui.available_width();

        ui.allocate_ui_with_layout(
            egui::vec2(width, 38.0),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                ui.painter().rect_filled(
                    ui.max_rect(),
                    0.0,
                    TITLE_BACKGROUND,
                );

                ui.add_space(10.0);

                ui.add(
                    egui::Image::new(&self.icon)
                        .fit_to_exact_size(egui::vec2(25.0, 25.0)),
                );

                ui.add_space(8.0);

                ui.label(
                    egui::RichText::new("Aurum")
                        .size(14.0)
                        .strong()
                        .color(TEXT_COLOR),
                );

                let drag_width =
                    (ui.available_width() - 140.0).max(0.0);

                let drag_response = ui.allocate_response(
                    egui::vec2(drag_width, 38.0),
                    egui::Sense::click_and_drag(),
                );

                if drag_response.drag_started_by(
                    egui::PointerButton::Primary,
                ) {
                    ui.ctx().send_viewport_cmd(
                        egui::ViewportCommand::StartDrag,
                    );
                }

                let maximized = ui.input(|input| {
                    input.viewport().maximized.unwrap_or(false)
                });

                if Self::title_button(
                    ui,
                    "—",
                    TEXT_COLOR,
                    HOVER_COLOR,
                ) {
                    ui.ctx().send_viewport_cmd(
                        egui::ViewportCommand::Minimized(true),
                    );
                }

                if Self::title_button(
                    ui,
                    if maximized { "❐" } else { "□" },
                    TEXT_COLOR,
                    HOVER_COLOR,
                ) {
                    ui.ctx().send_viewport_cmd(
                        egui::ViewportCommand::Maximized(!maximized),
                    );
                }

                if Self::title_button(
                    ui,
                    "×",
                    egui::Color32::from_rgb(255, 245, 220),
                    egui::Color32::from_rgb(190, 45, 35),
                ) {
                    ui.ctx().send_viewport_cmd(
                        egui::ViewportCommand::Close,
                    );
                }

                ui.add_space(7.0);
            },
        );
    }

    fn terminal(&mut self, ui: &mut egui::Ui) {
        let available = ui.available_size();

        egui::ScrollArea::vertical()
            .id_salt("terminal_output")
            .stick_to_bottom(true)
            .auto_shrink([false, false])
            .max_height((available.y - 32.0).max(100.0))
            .show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut self.output)
                        .font(egui::FontId::monospace(15.0))
                        .text_color(TEXT_COLOR)
                        .desired_width(f32::INFINITY)
                        .interactive(false)
                        .frame(egui::Frame::NONE),
                );
            });

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;

            ui.label(
                egui::RichText::new(
                    self.current_dir.display().to_string(),
                )
                .font(egui::FontId::monospace(15.0))
                .color(TEXT_COLOR),
            );

            ui.label(
                egui::RichText::new(">")
                    .font(egui::FontId::monospace(15.0))
                    .color(TEXT_COLOR),
            );

            let response = ui.add(
                egui::TextEdit::singleline(&mut self.command)
                    .font(egui::FontId::monospace(15.0))
                    .text_color(TEXT_COLOR)
                    .desired_width(f32::INFINITY)
                    .vertical_align(egui::Align::Center)
                    .frame(egui::Frame::NONE)
                    .margin(egui::Margin::ZERO),
            );

            if !response.has_focus() {
                response.request_focus();
            }

            if response.has_focus()
                && ui.input(|input| {
                    input.key_pressed(egui::Key::Enter)
                })
            {
                self.execute_command();
            }

            if response.has_focus()
                && ui.input(|input| {
                    input.key_pressed(egui::Key::ArrowUp)
                })
            {
                self.navigate_history(-1);
            }

            if response.has_focus()
                && ui.input(|input| {
                    input.key_pressed(egui::Key::ArrowDown)
                })
            {
                self.navigate_history(1);
            }
        });
    }
}

impl eframe::App for AurumApp {
    fn ui(
        &mut self,
        ui: &mut egui::Ui,
        _frame: &mut eframe::Frame,
    ) {
        let ctx = ui.ctx().clone();

        ui.painter().rect_filled(
            ui.max_rect(),
            0.0,
            BACKGROUND,
        );

        ui.with_layout(
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);

                self.title_bar(ui);

                ui.add_space(10.0);

                ui.allocate_ui_with_layout(
                    ui.available_size(),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        ui.add_space(5.0);
                        self.terminal(ui);
                    },
                );
            },
        );

        ctx.request_repaint_after(Duration::from_millis(50));
    }
}
