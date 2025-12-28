use crate::ui::landing::LandingPage;
use crate::ui::page::{Page, PageLayout};

/// Identifiant logique des pages existantes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Navigation {
    Landing,
}

impl Navigation {
    /// Toutes les pages disponibles
    pub fn all() -> &'static [Navigation] {
        &[Navigation::Landing]
    }

    /// Nom affiché dans la navigation
    pub fn name(&self) -> &'static str {
        match self {
            Navigation::Landing => "Dashboard",
        }
    }
}

pub struct NavigationManager {
    current: Navigation,
    landing: LandingPage,
}

impl NavigationManager {
    pub fn new() -> Self {
        Self {
            current: Navigation::Landing,
            landing: LandingPage::new(),
        }
    }

    /// Change la page actuelle
    pub fn navigate_to(&mut self, page: Navigation) {
        self.current = page;
    }

    /// Retourne la page actuelle
    pub fn current(&self) -> Navigation {
        self.current
    }

    /// Layout de la page actuelle
    pub fn current_layout(&self) -> PageLayout {
        match self.current {
            Navigation::Landing => self.landing.layout(),
        }
    }

    /// Affiche la page actuelle
    pub fn show_current(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        match self.current {
            Navigation::Landing => self.landing.show(ctx, ui),
        }
    }
}

impl Default for NavigationManager {
    fn default() -> Self {
        Self::new()
    }
}

