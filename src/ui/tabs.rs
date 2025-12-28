use eframe::egui;
use crate::ui::page::Page;

/// Page avec un système d'onglets
pub struct TabsPage {
    selected_tab: usize,
}

impl TabsPage {
    pub fn new() -> Self {
        Self {
            selected_tab: 0,
        }
    }
}

impl Page for TabsPage {
    fn show(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::Frame::NONE
            .fill(egui::Color32::from_rgb(40, 40, 55))
            .corner_radius(egui::CornerRadius::same(16))
            .inner_margin(egui::Margin::same(20))
            .show(ui, |ui| {
                ui.heading("📊 Projects");
                ui.separator();
                ui.add_space(10.0);
                
                // Barre d'onglets
                egui::Frame::NONE
                    .fill(egui::Color32::from_rgb(30, 30, 38))
                    .corner_radius(egui::CornerRadius::same(8))
                    .inner_margin(egui::Margin::same(8))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            let tabs = ["Tab 1", "Tab 2", "Tab 3"];
                            
                            for (idx, tab_name) in tabs.iter().enumerate() {
                                let is_selected = self.selected_tab == idx;
                                
                                let button = egui::Button::new(*tab_name)
                                    .fill(if is_selected {
                                        egui::Color32::from_rgb(60, 60, 80)
                                    } else {
                                        egui::Color32::TRANSPARENT
                                    });
                                
                                if ui.add(button).clicked() {
                                    self.selected_tab = idx;
                                }
                            }
                        });
                    });
                
                ui.add_space(16.0);
                
                // Contenu de l'onglet sélectionné
                match self.selected_tab {
                    0 => {
                        ui.heading("Contenu de l'onglet 1");
                        ui.separator();
                        ui.label("Ceci est le contenu du premier onglet.");
                        ui.add_space(10.0);
                        if ui.button("Action 1").clicked() {
                            println!("Action 1 cliquée!");
                        }
                    }
                    1 => {
                        ui.heading("Contenu de l'onglet 2");
                        ui.separator();
                        ui.label("Ceci est le contenu du deuxième onglet.");
                        ui.add_space(10.0);
                        if ui.button("Action 2").clicked() {
                            println!("Action 2 cliquée!");
                        }
                    }
                    2 => {
                        ui.heading("Contenu de l'onglet 3");
                        ui.separator();
                        ui.label("Ceci est le contenu du troisième onglet.");
                        ui.add_space(10.0);
                        if ui.button("Action 3").clicked() {
                            println!("Action 3 cliquée!");
                        }
                    }
                    _ => {}
                }
            });
    }
    
    fn name(&self) -> &'static str {
        "Projects"
    }
}

