pub mod module;
pub mod interface;

use egui_tiles::UiResponse;
use crate::gui::ui::editor::symbols::module::Module;
use crate::prelude::*;
use crate::gui::ui::panes::Pane;

#[derive(Default)]
pub struct Editor {
    module: ligen_symbols::module::Module
}

impl Editor {
    // TODO: Use this in the MenuButton
    // pub fn new(module: ligen_symbols::module::Module) -> Self {
    //     Self { module }
    // }
}

impl Pane for Editor {
    fn title(&self) -> String {
        self.module.identifier.to_string()
    }

    fn show(&mut self, ui: &mut egui::Ui) -> UiResponse {
        Module::new().show(ui, &mut self.module);
        UiResponse::None
    }
}