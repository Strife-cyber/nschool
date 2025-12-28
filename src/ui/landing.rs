use eframe::egui;
use crate::ui::page::Page;

/// Page d'accueil avec 2 panneaux
pub struct LandingPage;

impl LandingPage {
    pub fn new() -> Self {
        Self
    }
}

impl Page for LandingPage {
    fn show(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        let total_width = ui.available_width();
        let separator_width = 12.0;
        let panel_width = (total_width - separator_width) * 0.5;

        ui.horizontal(|ui| {
            // LEFT PANEL
            egui::Frame::none()
                .fill(egui::Color32::from_rgb(30, 30, 38))
                .corner_radius(egui::CornerRadius::same(14))
                .inner_margin(egui::Margin::same(18))
                .shadow(egui::Shadow {
                    offset: [0, 4],
                    blur: 12,
                    spread: 0,
                    color: egui::Color32::from_black_alpha(120),
                })
                .show(ui, |ui| {
                    ui.set_min_width(panel_width);
                    ui.set_max_width(panel_width);

                    ui.heading("📁 Navigation");
                    ui.separator();

                    ui.label("Dashboard");
                    ui.label("Projects");
                    ui.label("Settings");
                });

            // CENTER DIVIDER
            ui.allocate_ui(
                egui::vec2(separator_width, ui.available_height()),
                |ui| {
                    let rect = ui.max_rect();
                    ui.painter().rect_filled(
                        rect,
                        egui::CornerRadius::same(6),
                        egui::Color32::from_rgba_unmultiplied(255, 255, 255, 14),
                    );
                },
            );

            // RIGHT PANEL
            egui::Frame::none()
                .fill(egui::Color32::from_rgb(40, 40, 55))
                .corner_radius(egui::CornerRadius::same(18))
                .inner_margin(egui::Margin::same(22))
                .shadow(egui::Shadow {
                    offset: [0, 6],
                    blur: 18,
                    spread: 0,
                    color: egui::Color32::from_black_alpha(150),
                })
                .show(ui, |ui| {
                    ui.set_min_width(panel_width);
                    ui.set_max_width(panel_width);

                    ui.heading("✨ Main Content");
                    ui.separator();

                    ui.label("This is where your main app logic lives.");
                    ui.add_space(12.0);

                    let _ = ui.button("Do Something Cool");
                });
        });
    }

    fn name(&self) -> &'static str {
        "Dashboard"
    }
}
