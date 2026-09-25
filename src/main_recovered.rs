use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("TEMPEST STUDiO MANAGER")
            .with_inner_size([1280.0, 780.0]),
        ..Default::default()
    };

    eframe::run_native(
        "tempest_studio_pipeline",
        options,
        Box::new(|cc| {
            let mut visuals = egui::Visuals::dark();
            visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(18, 19, 26);
            visuals.widgets.noninteractive.fg_stroke.color = egui::Color32::from_rgba_premultiplied(255, 255, 255, 12);
            visuals.widgets.inactive.bg_fill = egui::Color32::from_rgba_premultiplied(33, 35, 47, 175);
            visuals.widgets.inactive.fg_stroke.color = egui::Color32::from_rgba_premultiplied(255, 255, 255, 22);
            visuals.widgets.hovered.bg_fill = egui::Color32::from_rgba_premultiplied(48, 52, 72, 210);
            visuals.widgets.hovered.fg_stroke.color = egui::Color32::from_rgb(220, 38, 38);
            cc.egui_ctx.set_visuals(visuals);
            Box::new(TempestGlassApp::default())
        }),
    )
}

struct MediaCard {
    category: &\'static str,
    title: &\'static str,
    desc: &\'static str,
}

struct TempestGlassApp {
    active_tab: &\'static str,
    media_pool: Vec<MediaCard>,
}

impl Default for TempestGlassApp {
    fn default() -> Self {
        Self {
            active_tab: "Essentials",
            media_pool: vec![
                MediaCard { category: "TUTORIAL SERIES", title: "Get Started with Tempest", desc: "Five-part cinematic video series taking you through core production essentials." },
                MediaCard { category: "DOCUMENTATION", title: "Tempest for New Users", desc: "Designed for anyone completely new to the rendering engine framework." },
                MediaCard { category: "MINI MOVIE", title: "Your First Hour in Tempest", desc: "Step-by-step masterclass covering game creation from start to finish." },
                MediaCard { category: "DOCUMENTATION", title: "Editor Interface Overview", desc: "Master common UI panels, hotkeys, viewports, and custom toolbars." },
            ],
        }
    }
}

impl eframe::App for TempestGlassApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_navigation_bar")
            .frame(egui::Frame::none().fill(egui::Color32::from_rgb(220, 38, 38)))
            .show(ctx, |ui| {
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.colored_label(egui::Color32::WHITE, " ? TEMPEST STUDiO");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.small("v5.8.2-STABLE  |  PRO LAYER MODE");
                    });
                });
                ui.add_space(8.0);
            });

        egui::SidePanel::left("left_dock")
            .resizable(false)
            .default_width(220.0)
            .frame(egui::Frame::none().fill(egui::Color32::from_rgb(13, 14, 19)))
            .show(ctx, |ui| {
                ui.add_space(24.0);
                ui.label("  NAVIGATION");
                ui.add_space(10.0);
                let categories = ["Essentials", "Games", "Film, TV & Live Events", "Architecture"];
                for cat in categories {
                    if ui.selectable_label(self.active_tab == cat, format!("  ?? {}", cat)).clicked() {
                        self.active_tab = cat;
                    }
                    ui.add_space(6.0);
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);
            ui.heading("Resources for Absolute Beginners");
            ui.colored_label(egui::Color32::from_rgb(160, 165, 180), "Ready to start your journey?");
            ui.add_space(15.0);

            egui::Frame::none()
                .fill(egui::Color32::from_rgba_premultiplied(28, 30, 41, 200))
                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgba_premultiplied(255, 255, 255, 30)))
                .inner_margin(20.0)
                .rounding(8.0)
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.horizontal(|ui| {
                        let (video_frame, _) = ui.allocate_space(egui::vec2(500.0, 240.0));
                        ui.painter().rect_filled(video_frame, 6.0, egui::Color32::from_rgb(16, 17, 24));
                        ui.painter().text(video_frame.center(), egui::Align2::CENTER_CENTER, "? PLAY MINI MOVIE: CORE PREVIEW // COMPILATION TIME METRIC: 93.87s", egui::FontId::proportional(12.0), egui::Color32::WHITE);
                    });
                });
        });
    }
}
