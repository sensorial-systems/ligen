use crate::{prelude::*, gui::ui::panes::Pane};
use ligen_parser::ParserConfigSet;
use ligen_python_parser::{PythonParserConfig, PythonParser};

pub mod parser;
use ligen_rust_parser::library::RustLibraryParser;
pub use parser::*;

use crate::gui::ui::panes::PaneManager;

use super::{ir::Editor, widget::Widget, settings::Settings};

pub struct Parsers {
    parsers: Vec<Parser>
}

impl Default for Parsers {
    fn default() -> Self {
        let parsers = vec![
            // Parser::new(PythonParser::default()),
            Parser::new(RustLibraryParser::default()) // TODO: This is the Rust parser. We need to rename it and also make it parse from the project directory. It's currently using the Cargo.toml file.
        ];
        Self { parsers }
    }
}

impl Parsers {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Pane for Parsers {
    fn title(&self) -> String {
        "Parsers".to_string()
    }
    fn show(&mut self, ui: &mut ligen_gui_runtime::egui::Ui, pane_manager: &mut PaneManager) -> egui_tiles::UiResponse {
        let mut settings = Settings::default();
        settings.editor.editable_fields = true;
        for (index, parser) in self.parsers.iter_mut().enumerate() {
            ui.push_id(index, |ui| {
                parser.show(&settings, ui, pane_manager);
            });
        }
        Default::default()
    }
}
