pub mod config;
pub use config::*;

use crate::{prelude::*, gui::ui::{editor::{widget::Widget, settings::Settings, ir::Editor}, panes::PaneManager}};
use std::path::Path;
use ligen_parsing::parser::{self, Parser as ParserTrait, ParserConfigSet, ParserConfig, ParserConfigGet};
use ligen_python_parser::parser::{PythonParser, PythonParserConfig};

pub struct Parser {
    parser: Box<dyn for<'a> parser::Parser<&'a Path, Output = ligen_ir::Library>>,
    config: ParserConfig
}

impl Parser {
    pub fn new<T>(parser: T) -> Self
    where T: for<'a> parser::Parser<&'a Path, Output = ligen_ir::Library> + 'static
    {
        let config = parser.config();
        let parser = Box::new(parser);
        Self { parser, config }
    }
}

impl Widget for Parser {
    type Input = PaneManager;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, pane_manager: &mut PaneManager) {
        ui.indent("indent", |ui| {
            ui.add_space(16.0);
            ui.label(self.parser.name());
            config::ParserConfig::new().show(settings, ui, &mut self.config);
            if ui.button("Parse").clicked() {
                let entry = rfd::FileDialog::new()
                    .pick_folder();
                if let Some(entry) = entry {
                    stacker::grow(1024 * 1024 * 10, || {
                        let library = self.parser.parse(entry.as_path(), &self.config).unwrap();
                        pane_manager.new_pane(Box::new(Editor::new(library)));
                    });
                }
            }    
        });
    }
}