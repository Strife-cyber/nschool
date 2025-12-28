mod db;
mod ui;

use eframe::egui;
use crate::db::init_database;
use crate::ui::page::PageLayout;
use crate::ui::navigation::{NavigationManager, Navigation};

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
        // Global background
        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(egui::Color32::from_rgb(18, 18, 22)),
            )
            .show(ctx, |ui| {
                match self.navigation.current_layout() {
                    PageLayout::Fullscreen => {
                        // Fullscreen pages (e.g., Landing)
                        self.navigation.show_current(ctx, ui);
                    }
                    PageLayout::WithNavigation => {
                        // Pages with left navigation
                        ui.horizontal(|ui| {
                            // LEFT NAV PANEL
                            egui::Frame::new()
                                .fill(egui::Color32::from_rgb(30, 30, 38))
                                .corner_radius(egui::CornerRadius::same(12))
                                .inner_margin(egui::Margin::same(16))
                                .show(ui, |ui| {
                                    ui.set_min_width(250.0);

                                    ui.heading("📁 Navigation");
                                    ui.separator();

                                    // Navigation buttons
                                    for page in Navigation::all() {
                                        let is_current = self.navigation.current() == *page;

                                        let button = egui::Button::new(page.name())
                                            .fill(if is_current {
                                                egui::Color32::from_rgb(60, 60, 80)
                                            } else {
                                                egui::Color32::TRANSPARENT
                                            });

                                        if ui.add(button).clicked() {
                                            self.navigation.navigate_to(*page);
                                        }
                                    }
                                });

                            // VISUAL SEPARATOR
                            ui.add_space(16.0);

                            // MAIN CONTENT PANEL
                            egui::Frame::new()
                                .fill(egui::Color32::from_rgb(40, 40, 55))
                                .corner_radius(egui::CornerRadius::same(16))
                                .inner_margin(egui::Margin::same(20))
                                .show(ui, |ui| {
                                    self.navigation.show_current(ctx, ui);
                                });
                        });
                    }
                }
            });
    }
}
