mod db;
mod ui;

use eframe::egui;
use crate::db::init_database;
use crate::ui::navigation::{Navigation, NavigationManager};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _conn = init_database("database/nschool.sqlite")?;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 500.0])
            .with_title("Nschool - Student Management"),
        ..Default::default()
    };

    eframe::run_native(
        "Nschool - Student Management",
        options,
        Box::new(|_cc| Ok(Box::<Nschool>::default())),
    )?;

    Ok(())
}

#[derive(Default)]
struct Nschool {
    navigation: NavigationManager,
}

impl eframe::App for Nschool {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Fond d'écran global
        egui::CentralPanel::default()
            .frame(
                egui::Frame::NONE
                    .fill(egui::Color32::from_rgb(18, 18, 22)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // PANEL DE NAVIGATION GAUCHE
                    egui::Frame::NONE
                        .fill(egui::Color32::from_rgb(30, 30, 38))
                        .corner_radius(egui::CornerRadius::same(12))
                        .inner_margin(egui::Margin::same(16))
                        .show(ui, |ui| {
                            ui.set_min_width(250.0);

                            ui.heading("📁 Navigation");
                            ui.separator();

                            // Boutons de navigation
                            for page in Navigation::all() {
                                let is_current = self.navigation.current_page() == page;
                                let button_text = format!("{} {}", page.icon(), page.name());
                                
                                let button = egui::Button::new(button_text)
                                    .fill(if is_current {
                                        egui::Color32::from_rgb(60, 60, 80)
                                    } else {
                                        egui::Color32::TRANSPARENT
                                    });
                                
                                if ui.add(button).clicked() {
                                    self.navigation.navigate_to(page);
                                }
                            }
                        });

                    // SÉPARATEUR VISUEL
                    ui.add_space(16.0);

                    // PANEL DE CONTENU PRINCIPAL
                    egui::Frame::NONE
                        .fill(egui::Color32::from_rgb(40, 40, 55))
                        .corner_radius(egui::CornerRadius::same(16))
                        .inner_margin(egui::Margin::same(20))
                        .show(ui, |ui| {
                            // Afficher la page actuelle
                            self.navigation.show_current_page(ctx, ui);
                        });
                });
            });
    }
}