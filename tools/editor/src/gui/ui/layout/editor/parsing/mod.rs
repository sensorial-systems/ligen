use crate::{prelude::*, gui::ui::panes::Pane};
use ligen_parsing::parser::ParserConfigSet;
use ligen_python_parser::parser::{PythonParserConfig, PythonParser};

pub mod parser;
pub use parser::*;

use crate::gui::ui::panes::PaneManager;

use super::{ir::Editor, widget::Widget, settings::Settings};

pub struct Parsing {
    parsers: Vec<Parser>
}

impl Default for Parsing {
    fn default() -> Self {
        let parsers = vec![
            Parser::new(Box::new(PythonParser::default()))
        ];
        Self { parsers }
    }
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