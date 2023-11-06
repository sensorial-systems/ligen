pub use crate::prelude::*;

use egui::{Style, Visuals};
pub mod ui;

/// We derive Deserialize/Serialize so we can persist gui state on shutdown.
#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Gui {
    layout: ui::Layout,
}

impl Gui {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        let mut visuals = Visuals::light();
        visuals.indent_has_left_vline = false;
        cc.egui_ctx.set_style(Style { visuals, ..Default::default() });

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        cc
            .storage
            .and_then(|storage| eframe::get_value(storage, eframe::APP_KEY))
            .unwrap_or_default()
    }
}

impl eframe::App for Gui {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.layout.show(ctx, frame);
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}