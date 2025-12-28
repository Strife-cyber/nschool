use eframe::egui;

/// Trait pour toutes les pages de l'application
pub trait Page {
    /// Affiche le contenu de la page
    fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui);
    
    /// Retourne le nom de la page pour la navigation
    fn name(&self) -> &'static str;
}

