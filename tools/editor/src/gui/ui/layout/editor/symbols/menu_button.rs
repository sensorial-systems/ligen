use ligen_python_parser::symbols::module::ModuleParser;
use ligen_parsing::parser::Parser;
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
        if ui.button("Parse Python").clicked() {
            let file = rfd::FileDialog::new()
                .add_filter("Python File", &["py"])
                .pick_file();

            if let Some(file) = file {
                let content = std::fs::read_to_string(file).unwrap();
                let module = ModuleParser
                    .parse(content.as_str())
                    .expect("Failed to parse project.");
                panes.new_pane(Box::new(Editor::new(module)));
            }

            ui.close_menu();
        }
    }
}
