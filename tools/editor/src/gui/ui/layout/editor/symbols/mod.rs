pub mod module;
pub mod interface;
pub mod project;

pub mod menu_button;

use egui_tiles::UiResponse;
use ligen_ir::symbols::Symbols;
use crate::gui::ui::{EditableList, List};
use crate::gui::ui::editor::symbols::project::Project;
use crate::prelude::*;
use crate::gui::ui::panes::Pane;

#[derive(Default)]
pub struct Editor {
    project: ligen_ir::Project,
    symbols: Symbols,
    filter: String
}

impl Editor {
    pub fn new(project: ligen_ir::Project) -> Self {
        let mut symbols = Symbols::new(&project);
        let filter = String::new();
        Self { project, symbols, filter }
    }
}

impl Pane for Editor {
    fn title(&self) -> String {
        self.project.root_module.identifier.to_string()
    }

    fn show(&mut self, ui: &mut egui::Ui) -> UiResponse {
        Project::new().show(ui, &mut self.project.root_module);
        ui.text_edit_singleline(&mut self.filter);
        List::new("Symbols").show(ui, &mut self.symbols.symbols.iter_mut().filter(|symbol| symbol.to_string().contains(self.filter.as_str())), |ui, symbol| {
            ui.label(symbol.to_string());
        });
        UiResponse::None
    }
}
