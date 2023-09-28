mod editor;
mod menu;
mod panes;

use serde::{Deserialize, Serialize};
pub use editor::*;
pub use menu::*;
pub use panes::*;

#[derive(Serialize, Deserialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Layout {
    #[serde(skip)]
    menu: Menu,
    #[serde(skip)]
    panes: Panes
}


pub trait MenuButton {
    fn menu_title(&self) -> String;
    fn show_button(&self, ui: &mut egui::Ui, panes: &mut Panes);

    fn menu_button(&self, ui: &mut egui::Ui, panes: &mut Panes) {
        ui.menu_button(self.menu_title(), |ui| {
            self.show_button(ui, panes)
        });
    }
}

impl Default for Layout {
    fn default() -> Self {
        let panes = Panes::new();
        let menu = Menu::new();
        Self { menu, panes }
    }
}

impl Layout {
    pub fn show(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.menu.show(ctx, frame, &mut self.panes);
        self.panes.show(ctx);
    }
}