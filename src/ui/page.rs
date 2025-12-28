use eframe::egui;

/// How a page want to be laid out
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageLayout {
    /// Page takes the full window (no side navigation)
    Fullscreen,

    /// Page is shown with left navigation + content panel
    WithNavigation,
}

pub trait Page {
    /// Affiche le contenu de la page
    fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui);

    /// Nom affiché dans la navigation
    fn name(&self) -> &'static str;

    /// Type de layout souhaité par la page
    fn layout(&self) -> PageLayout {
        PageLayout::WithNavigation
    }
}
