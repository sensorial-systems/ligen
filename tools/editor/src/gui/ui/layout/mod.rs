pub mod editor;
pub mod menu;
pub mod panes;

use serde::{Deserialize, Serialize};
use crate::gui::ui::menu::Menu;
use crate::gui::ui::panes::Panes;
use crate::prelude::*;

use self::editor::ir::Editor;
use self::editor::parsing::Parsing;

#[derive(Serialize, Deserialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Layout {
    #[serde(skip)]
    menu: Menu,
    #[serde(skip)]
    panes: Panes
}

impl Default for Layout {
    fn default() -> Self {
        let menu = Menu::new();
        let mut panes = Panes::new();
        panes.new_pane(Box::new(Parsing::new()));
        Self { menu, panes }
    }
}

impl Layout {
    pub fn show(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.menu.show(ctx, frame, &mut self.panes);
        self.panes.show(ctx);
    }
}
