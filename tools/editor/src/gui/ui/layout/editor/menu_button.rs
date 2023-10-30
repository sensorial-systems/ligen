use ligen_ir::Project;
use ligen_python_parser::parser::PythonParser;

use crate::prelude::*;
use crate::gui::ui::editor::ir::Editor;
use crate::gui::ui::menu::MenuButton;
use crate::gui::ui::panes::Panes;
use ligen_parsing::parser::Parser;

pub struct EditorMenuButton;
impl MenuButton for EditorMenuButton {
    fn menu_title(&self) -> String {
        "Project".to_string()
    }
    fn show_button(&self, ui: &mut egui::Ui, panes: &mut Panes) {
        if ui.button("Open").clicked() {
            let file = rfd::FileDialog::new()
                .add_filter("ligen-ir", &["lir"])
                .pick_file();
            if let Some(file) = file {
                if let Ok(project) = ligen_ir::Project::load(file) {
                    panes.new_pane(Box::new(Editor::new(project)));
                }
            }
            ui.close_menu();
        }
        if ui.button("Parse Python").clicked() {
            let entry = rfd::FileDialog::new()
                .pick_folder();

            if let Some(entry) = entry {
                stacker::grow(1024 * 1024 * 10, || {
                    let symbols = PythonParser::symbol().parse(entry.as_path()).unwrap();
                    let full = PythonParser::full().parse(entry.as_path()).unwrap();
                    panes.new_pane(Box::new(Editor::new(symbols)));
                    panes.new_pane(Box::new(Editor::new(full)));
                });
            }

            ui.close_menu();
        }
        if ui.button("Parse Rust/Cargo").clicked() {
            use ligen_cargo::parser::project::ProjectParser;

            let file = rfd::FileDialog::new()
                .add_filter("Cargo project", &["toml"])
                .pick_file();

            if let Some(file) = file {
                let project = ProjectParser
                    .parse(file.as_path())
                    .expect("Failed to parse project.");
                panes.new_pane(Box::new(Editor::new(project)));
            }

            ui.close_menu();
        }
    }
}