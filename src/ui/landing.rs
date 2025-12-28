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
        ui.horizontal(|ui| {
            // LEFT PANEL
            egui::Frame::NONE
                .fill(egui::Color32::from_rgb(30, 30, 38))
                .corner_radius(egui::CornerRadius::same(12))
                .inner_margin(egui::Margin::same(16))
                .show(ui, |ui| {
                    ui.set_min_width(250.0);

                    ui.heading("📁 Navigation");
                    ui.separator();

                    ui.label("• Dashboard");
                    ui.label("• Projects");
                    ui.label("• Settings");
                });

            // VISUAL SEPARATOR
            ui.add_space(16.0);

            // RIGHT PANEL
            egui::Frame::NONE
                .fill(egui::Color32::from_rgb(40, 40, 55))
                .corner_radius(egui::CornerRadius::same(16))
                .inner_margin(egui::Margin::same(20))
                .show(ui, |ui| {
                    ui.heading("✨ Main Content");
                    ui.separator();

                    ui.label("This is where your main app logic lives.");
                    ui.add_space(10.0);

                    if ui.button("Do Something Cool").clicked() {
                        println!("Clicked!");
                    }
                });
        });
    }
    
    fn name(&self) -> &'static str {
        "Dashboard"
    }
}

