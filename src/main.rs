use std::fmt::format;
use eframe::{run_native, App, Frame, NativeOptions};
use eframe::egui::{CentralPanel, ScrollArea, Vec2, Context};
use egui::{FontId, TextStyle};

#[derive(Default)]
struct Feeds {
    episodes: Vec<Episode>
}

struct Episode {
    title: String,
    link: String,
    desc: String
}

impl App for Feeds {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().max_width(f32::INFINITY).auto_shrink([false; 2]).show(ui, |ui| {
                self.episodes = self.data().collect();
                for ep in &self.episodes {
                    if ui.button(&ep.title).clicked() {
                        test_print(&ep.title)
                    };
                    ui.hyperlink(&ep.link);
                }
            });
        });
    }
}

impl Feeds {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        Self::default()
    }
    fn data(&self) -> impl  Iterator<Item = Episode> {
        (0..20).map(|a| Episode {
            title: format!("Title: {}", a),
            link: format!("https://mikee.site/{}", a),
            desc: format!("Desc: {}", a)
        })
    }
}

fn test_print(s: &str) {
    println!("{}", s);
}

fn setup_custom_fonts(ctx: &Context) {
//     use the regular fonts first
    let mut fonts = egui::FontDefinitions::default();
//     add some other fonts
    fonts.font_data.insert(
        "fantasque".to_owned(), egui::FontData::from_static(include_bytes!(
            "../assets/Fantasque Sans Mono Regular Nerd Font Complete Mono.ttf"
        )),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "fantasque".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "fantasque".to_owned());
    ctx.set_fonts(fonts);
    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Body, FontId::new(36.0, egui::FontFamily::Proportional)),
        (TextStyle::Button, FontId::new(32.0, egui::FontFamily::Proportional)),
    ].into();
    ctx.set_style(style);
}



fn main() {
    let app: Feeds = Feeds {
        episodes: Vec::new()
    };
    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(
        540.,
        960.
    ));
    run_native("Feeds", win_option, Box::new(|cc| Box::new(Feeds::new(cc))) );
}

