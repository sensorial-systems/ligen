use ligen_idl::Library;
use ligen_python_parser::{PythonParser, PythonParserConfig};

use crate::prelude::*;
use crate::gui::ui::editor::ir::Editor;
use crate::gui::ui::menu::MenuButton;
use crate::gui::ui::panes::Panes;
use ligen_transformer::prelude::*;

pub struct EditorMenuButton;
impl MenuButton for EditorMenuButton {
    fn menu_title(&self) -> String {
        "Library".to_string()
    }
    fn show_button(&self, ui: &mut egui::Ui, panes: &mut Panes) {
        if ui.button("Open").clicked() {
            let file = rfd::FileDialog::new()
                .add_filter("ligen-idl", &["lir"])
                .pick_file();
            if let Some(file) = file {
                if let Ok(library) = ligen_idl::Library::load(file) {
                    panes.new_pane(Box::new(Editor::new(library)));
                }
            }
            ui.close_menu();
        }
    }
}
