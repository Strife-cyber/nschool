use eframe::egui;
use crate::ui::page::{Page, PageLayout};

pub struct LandingPage;

impl LandingPage {
    pub fn new() -> Self {
        Self
    }
}

impl Page for LandingPage {
    fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        // your existing two-panel UI here
    }

    fn name(&self) -> &'static str {
        "Dashboard"
    }

    fn layout(&self) -> PageLayout {
        PageLayout::Fullscreen
    }
}
