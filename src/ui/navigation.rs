use crate::ui::page::Page;
use crate::ui::landing::LandingPage;
use crate::ui::tabs::TabsPage;

/// Enum pour gérer la navigation entre les pages
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Navigation {
    Landing,
    Tabs,
}

impl Navigation {
    /// Retourne toutes les pages disponibles
    pub fn all() -> Vec<Navigation> {
        vec![Navigation::Landing, Navigation::Tabs]
    }
    
    /// Retourne le nom de la page
    pub fn name(&self) -> &'static str {
        match self {
            Navigation::Landing => "Dashboard",
            Navigation::Tabs => "Projects",
        }
    }
    
    /// Retourne l'icône de la page
    pub fn icon(&self) -> &'static str {
        match self {
            Navigation::Landing => "🏠",
            Navigation::Tabs => "📊",
        }
    }
}

/// Gestionnaire de navigation qui maintient l'état des pages
pub struct NavigationManager {
    current_page: Navigation,
    landing_page: LandingPage,
    tabs_page: TabsPage,
}

impl NavigationManager {
    pub fn new() -> Self {
        Self {
            current_page: Navigation::Landing,
            landing_page: LandingPage::new(),
            tabs_page: TabsPage::new(),
        }
    }
    
    /// Change la page actuelle
    pub fn navigate_to(&mut self, page: Navigation) {
        self.current_page = page;
    }
    
    /// Retourne la page actuelle
    pub fn current_page(&self) -> Navigation {
        self.current_page
    }
    
    /// Affiche la page actuelle
    pub fn show_current_page(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        match self.current_page {
            Navigation::Landing => self.landing_page.show(ctx, ui),
            Navigation::Tabs => self.tabs_page.show(ctx, ui),
        }
    }
}

impl Default for NavigationManager {
    fn default() -> Self {
        Self::new()
    }
}

