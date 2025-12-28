use eframe::egui;
use eframe::epaint::{ColorImage, TextureHandle};
use crate::ui::page::{Page, PageLayout};
use std::time::{Duration, Instant};

pub struct LandingPage {
    username: String,
    password: String,
    carousel_images: Vec<TextureHandle>,
    current_image_index: usize,
    last_image_change: Instant,
    image_urls: Vec<&'static str>,
    image_colors: Vec<egui::Color32>,
}

impl LandingPage {
    pub fn new() -> Self {
        // URLs d'images de personnes étudiant (Unsplash) - pour référence future
        let image_urls = vec![
            "https://images.unsplash.com/photo-1522202176988-66273c2fd55f?w=800&h=600&fit=crop",
            "https://images.unsplash.com/photo-1503676260728-1c00da094a0b?w=800&h=600&fit=crop",
            "https://images.unsplash.com/photo-1523050854058-8df90110c9f1?w=800&h=600&fit=crop",
            "https://images.unsplash.com/photo-1516321318423-f06f85e504b3?w=800&h=600&fit=crop",
            "https://images.unsplash.com/photo-1524178232363-1fb2b075b655?w=800&h=600&fit=crop",
        ];

        // Couleurs pour les placeholders (sera remplacé par de vraies images)
        let image_colors = vec![
            egui::Color32::from_rgb(60, 80, 120),   // Bleu foncé
            egui::Color32::from_rgb(80, 100, 140),  // Bleu moyen
            egui::Color32::from_rgb(100, 120, 160), // Bleu clair
            egui::Color32::from_rgb(70, 90, 130),   // Bleu-gris
            egui::Color32::from_rgb(90, 110, 150), // Bleu-gris clair
        ];

        Self {
            username: String::new(),
            password: String::new(),
            carousel_images: Vec::new(),
            current_image_index: 0,
            last_image_change: Instant::now(),
            image_urls,
            image_colors,
        }
    }

    /// Crée une texture placeholder colorée
    fn create_placeholder_texture(ctx: &egui::Context, color: egui::Color32, index: usize) -> TextureHandle {
        let size = [800, 600];
        let pixels = vec![color; size[0] * size[1]];
        let color_image = ColorImage {
            size,
            pixels,
            source_size: egui::vec2(size[0] as f32, size[1] as f32),
        };
        ctx.load_texture(format!("carousel_image_{}", index), color_image, Default::default())
    }

    /// Initialise les images du carousel
    pub fn initialize_images(&mut self, ctx: &egui::Context) {
        if self.carousel_images.is_empty() {
            for (i, &color) in self.image_colors.iter().enumerate() {
                let texture = Self::create_placeholder_texture(ctx, color, i);
                self.carousel_images.push(texture);
            }
        }
    }

    /// Met à jour le carousel (change d'image toutes les 10 secondes)
    fn update_carousel(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_image_change) >= Duration::from_secs(10) {
            if !self.carousel_images.is_empty() {
                self.current_image_index = (self.current_image_index + 1) % self.carousel_images.len();
            }
            self.last_image_change = now;
        }
    }

    /// Affiche le formulaire de connexion
    fn show_login_form(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(60.0);
            
            // Logo/Titre
            ui.heading(egui::RichText::new("🎓 Nschool").size(42.0).strong().color(egui::Color32::from_rgb(100, 150, 255)));
            ui.add_space(8.0);
            ui.label(egui::RichText::new("Student Management System").size(16.0).color(egui::Color32::LIGHT_GRAY));
            ui.add_space(50.0);

            // Formulaire de connexion
            egui::Frame::NONE
                .fill(egui::Color32::from_rgb(35, 35, 50))
                .corner_radius(egui::CornerRadius::same(16))
                .inner_margin(egui::Margin::same(32))
                .outer_margin(egui::Margin::same(20))
                .show(ui, |ui| {
                    ui.set_width(320.0);
                    
                    ui.vertical(|ui| {
                        ui.heading(egui::RichText::new("Connexion").size(24.0).strong());
                        ui.add_space(30.0);

                        // Champ username
                        ui.label(egui::RichText::new("Nom d'utilisateur").size(14.0));
                        ui.add_space(8.0);
                        let _username_response = ui.add_sized(
                            [ui.available_width(), 40.0],
                            egui::TextEdit::singleline(&mut self.username)
                                .hint_text("Entrez votre nom d'utilisateur")
                                .font(egui::TextStyle::Body)
                        );
                        ui.add_space(20.0);

                        // Champ password
                        ui.label(egui::RichText::new("Mot de passe").size(14.0));
                        ui.add_space(8.0);
                        let _password_response = ui.add_sized(
                            [ui.available_width(), 40.0],
                            egui::TextEdit::singleline(&mut self.password)
                                .hint_text("Entrez votre mot de passe")
                                .password(true)
                                .font(egui::TextStyle::Body)
                        );
                        ui.add_space(30.0);

                        // Bouton de connexion
                        let login_button = ui.add_sized(
                            [ui.available_width(), 45.0],
                            egui::Button::new(egui::RichText::new("Se connecter").size(16.0).strong())
                                .fill(egui::Color32::from_rgb(100, 150, 255))
                        );

                        if login_button.clicked() {
                            println!("Tentative de connexion: username={}, password={}", self.username, self.password);
                            // TODO: Implémenter la logique de connexion
                        }

                        ui.add_space(20.0);
                        
                        // Lien "Mot de passe oublié"
                        ui.horizontal_centered(|ui| {
                            if ui.link("Mot de passe oublié ?").clicked() {
                                println!("Mot de passe oublié cliqué");
                            }
                        });
                    });
                });

            ui.add_space(40.0);
            
            // Footer
            ui.label(egui::RichText::new("© 2024 Nschool. Tous droits réservés.").size(12.0).color(egui::Color32::DARK_GRAY));
        });
    }

    /// Affiche le carousel d'images
    fn show_carousel(&mut self, ui: &mut egui::Ui, available_rect: egui::Rect) {
        let image_rect = available_rect.shrink(40.0);
        
        // Afficher l'image actuelle si disponible
        if let Some(texture) = self.carousel_images.get(self.current_image_index) {
            let image_size = texture.size_vec2();
            let scale = (image_rect.width() / image_size.x).min(image_rect.height() / image_size.y);
            let scaled_size = image_size * scale;
            let image_pos = egui::pos2(
                image_rect.center().x - scaled_size.x / 2.0,
                image_rect.center().y - scaled_size.y / 2.0,
            );
            
            let image_rect_final = egui::Rect::from_min_size(image_pos, scaled_size);
            
            // Fond avec coins arrondis
            ui.painter().rect_filled(
                image_rect_final,
                egui::CornerRadius::same(20),
                egui::Color32::from_rgb(40, 50, 70),
            );
            
            // Afficher la texture
            ui.put(
                image_rect_final,
                egui::Image::new((texture.id(), scaled_size)),
            );
            
            // Overlay avec texte
            ui.allocate_ui_at_rect(image_rect_final, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(image_rect_final.height() * 0.3);
                    ui.label(egui::RichText::new("📚").size(60.0));
                    ui.add_space(15.0);
                    ui.label(egui::RichText::new("Étudiez avec Nschool").size(28.0).strong().color(egui::Color32::WHITE));
                    ui.add_space(8.0);
                    ui.label(egui::RichText::new("Votre plateforme d'apprentissage").size(16.0).color(egui::Color32::LIGHT_GRAY));
                });
            });
        } else {
            // Placeholder si pas d'image
            ui.allocate_ui_at_rect(image_rect, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(image_rect.height() * 0.4);
                    ui.label(egui::RichText::new("📚").size(80.0));
                    ui.add_space(20.0);
                    ui.label(egui::RichText::new("Étudiez avec Nschool").size(24.0).color(egui::Color32::LIGHT_GRAY));
                });
            });
        }

        // Indicateurs de progression (points en bas)
        let indicator_y = image_rect.max.y - 30.0;
        let indicator_spacing = 12.0;
        let total_width = (self.carousel_images.len() as f32) * indicator_spacing;
        let start_x = image_rect.center().x - total_width / 2.0;

        for (i, _) in self.carousel_images.iter().enumerate() {
            let x = start_x + (i as f32) * indicator_spacing;
            let is_active = i == self.current_image_index;
            let color = if is_active {
                egui::Color32::from_rgb(100, 150, 255)
            } else {
                egui::Color32::from_rgb(80, 80, 100)
            };
            
            ui.painter().circle_filled(
                egui::pos2(x, indicator_y),
                if is_active { 6.0 } else { 4.0 },
                color,
            );
        }
    }
}

impl Page for LandingPage {
    fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        // Initialiser les images si nécessaire
        self.initialize_images(ctx);
        
        // Mettre à jour le carousel
        self.update_carousel();
        
        // Demander un repaint toutes les secondes pour le carousel
        ctx.request_repaint_after(Duration::from_secs(1));

        let available_rect = ui.available_rect_before_wrap();
        let left_width = 450.0;
        let right_width = available_rect.width() - left_width;

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;

            // --- PANEL GAUCHE: Formulaire de connexion ---
            egui::Frame::NONE
                .fill(egui::Color32::from_rgb(25, 25, 35))
                .show(ui, |ui| {
                    ui.set_min_width(left_width);
                    ui.set_max_width(left_width);
                    
                    self.show_login_form(ui);
                });

            // --- PANEL DROIT: Carousel d'images ---
            egui::Frame::NONE
                .fill(egui::Color32::from_rgb(30, 35, 45))
                .show(ui, |ui| {
                    ui.set_min_width(right_width);
                    
                    let carousel_rect = ui.available_rect_before_wrap();
                    self.show_carousel(ui, carousel_rect);
                });
        });
    }

    fn name(&self) -> &'static str { "Landing" }
    fn layout(&self) -> PageLayout { PageLayout::Fullscreen }
}
