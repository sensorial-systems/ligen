use ligen_parsing::parser::{ParserConfig, Parser, ParserConfigSet};
use ligen_python_parser::parser::{PythonParser, PythonParserConfig};

use self::ir::Editor;

use super::panes::{Pane, PaneManager};
use crate::prelude::*;

pub mod ir;
pub mod settings;
pub mod widget;
pub mod menu_button;
pub mod parsing;

#[derive(Default)]
pub struct Parsing {

}

impl Parsing {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Pane for Parsing {
    fn title(&self) -> String {
        "Parsing".to_string()
    }
    fn show(&mut self, ui: &mut ligen_gui_runtime::egui::Ui, pane_manager: &mut PaneManager) -> egui_tiles::UiResponse {
        ui.indent("indent", |ui| {
            ui.add_space(16.0);
            if ui.button("Parse Python").clicked() {
                let entry = rfd::FileDialog::new()
                    .pick_folder();
    
                if let Some(entry) = entry {
                    stacker::grow(1024 * 1024 * 10, || {
                        let mut full_config = PythonParserConfig::new();
                        full_config.set_class_variables_as_properties(true);
                        let mut symbols_config = full_config.clone();
                        symbols_config.set_only_parse_symbols(true);
                        let parser = PythonParser::default();
                        let symbols = parser.parse(entry.as_path(), &symbols_config).unwrap();
                        let full = parser.parse(entry.as_path(), &full_config).unwrap();
                        pane_manager.new_pane(Box::new(Editor::new(symbols)));
                        pane_manager.new_pane(Box::new(Editor::new(full)));
                    });
                }
            }    
        });
        
        Default::default()
    }
}