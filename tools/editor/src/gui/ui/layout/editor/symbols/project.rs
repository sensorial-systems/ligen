pub use crate::prelude::*;

use crate::gui::ui::editor::symbols::module::Module;

pub struct Project;

impl Project {
    pub fn new() -> Self {
        Self
    }

    pub fn show(&mut self, ui: &mut egui::Ui, module: &mut ligen_symbols::Module) {
        egui::ScrollArea::both()
            .auto_shrink([false, true])
            .show(ui, |ui| {
                Module::new().show(ui, module);
            });
    }
}
