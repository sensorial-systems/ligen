use ligen_ir::Library;
use ligen_python_parser::parser::PythonParser;

use crate::prelude::*;
use crate::gui::ui::editor::ir::Editor;
use crate::gui::ui::menu::MenuButton;
use crate::gui::ui::panes::Panes;
use ligen_parsing::parser::Parser;

pub struct EditorMenuButton;
impl MenuButton for EditorMenuButton {
    fn menu_title(&self) -> String {
        "Library".to_string()
    }
    fn show_button(&self, ui: &mut egui::Ui, panes: &mut Panes) {
        if ui.button("Open").clicked() {
            let file = rfd::FileDialog::new()
                .add_filter("ligen-ir", &["lir"])
                .pick_file();
            if let Some(file) = file {
                if let Ok(library) = ligen_ir::Library::load(file) {
                    panes.new_pane(Box::new(Editor::new(library)));
                }
            }
            ui.close_menu();
        }
        if ui.button("Parse Python").clicked() {
            let entry = rfd::FileDialog::new()
                .pick_folder();

            if let Some(entry) = entry {
                stacker::grow(1024 * 1024 * 10, || {
                    let config = Default::default();
                    let symbols = PythonParser::symbol().parse(entry.as_path(), &config).unwrap();
                    let full = PythonParser::full().parse(entry.as_path(), &config).unwrap();
                    panes.new_pane(Box::new(Editor::new(symbols)));
                    panes.new_pane(Box::new(Editor::new(full)));
                });
            }

            ui.close_menu();
        }
        if ui.button("Parse Rust/Cargo").clicked() {
            use ligen_cargo::parser::library::LibraryParser;

            let file = rfd::FileDialog::new()
                .add_filter("Cargo library", &["toml"])
                .pick_file();

            if let Some(file) = file {
                let library = LibraryParser
                    .parse(file.as_path(), &Default::default())
                    .expect("Failed to parse library.");
                panes.new_pane(Box::new(Editor::new(library)));
            }

            ui.close_menu();
        }
    }
}
