use ligen_ir::Project;
use ligen_parsing::parser::Parser;
use ligen_python_parser::module::ModuleParser;
use ligen_python_parser::parser::PythonParser;
use crate::prelude::*;
use crate::gui::ui::editor::symbols::Editor;
use crate::gui::ui::menu::MenuButton;
use crate::gui::ui::panes::Panes;

pub struct EditorMenuButton;
impl MenuButton for EditorMenuButton {
    fn menu_title(&self) -> String {
        "Symbols".to_string()
    }
    fn show_button(&self, ui: &mut egui::Ui, panes: &mut Panes) {
        if ui.button("Parse Python folder").clicked() {
            let entry = rfd::FileDialog::new()
                .pick_folder();

            if let Some(entry) = entry {
                stacker::grow(1024 * 1024 * 10, || {
                    let root_module = PythonParser::symbol().parse(entry.as_path()).unwrap();
                    let project = Project { root_module, ..Default::default() };
                    panes.new_pane(Box::new(Editor::new(project)));
                });
            }

            ui.close_menu();
        }
    }
}
