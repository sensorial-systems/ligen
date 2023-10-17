pub mod module;
pub mod interface;
pub mod project;

pub mod menu_button;

use egui_tiles::UiResponse;
use crate::gui::ui::editor::symbols::project::Project;
use crate::prelude::*;
use crate::gui::ui::panes::Pane;

#[derive(Default)]
pub struct Editor {
    module: ligen_symbols::Module
}

impl Editor {
    pub fn new(module: ligen_symbols::Module) -> Self {
        Self { module }
    }
}

impl Pane for Editor {
    fn title(&self) -> String {
        self.module.identifier.to_string()
    }

    fn show(&mut self, ui: &mut egui::Ui) -> UiResponse {
        Project::new().show(ui, &mut self.module);
        UiResponse::None
    }
}