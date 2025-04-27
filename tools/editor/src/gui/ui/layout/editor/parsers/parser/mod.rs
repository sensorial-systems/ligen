pub mod config;
pub use config::*;
use ligen_gui_runtime::egui::{CollapsingHeader, Color32};

use crate::{prelude::*, gui::ui::{editor::{widget::Widget, settings::Settings, ir::Editor}, panes::PaneManager}};
use std::path::Path;
use ligen_parser::{self, prelude::{Parser as ParserTrait, ConfigSet, ConfigGet, Config}};

use ligen_python_parser::{PythonParser, PythonParserConfig};

pub struct Parser {
    parser: Box<dyn for<'a> ligen_parser::prelude::Transformer<&'a Path, ligen_ir::Library>>,
    config: Config,
    result: String
}

impl Parser {
    pub fn new<T>(parser: T) -> Self
    where T: for<'a> ligen_parser::prelude::Transformer<&'a Path, ligen_ir::Library> + 'static
    {
        let config = parser.config();
        let parser = Box::new(parser);
        let result = Default::default();
        Self { parser, config, result }
    }
}

impl Widget for Parser {
    type Input = PaneManager;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, pane_manager: &mut PaneManager) {
        CollapsingHeader::new(self.parser.name())
            .default_open(false)
            .show(ui, |ui| {
                config::ParserConfig::new().show(settings, ui, &mut self.config);
                if ui.button("Parse").clicked() {
                    let entry = rfd::FileDialog::new()
                        .pick_folder();
                    if let Some(entry) = entry {
                        stacker::grow(1024 * 1024 * 10, || {
                            match self.parser.transform(entry.as_path(), &self.config) {
                                Ok(library) => pane_manager.new_pane(Box::new(Editor::new(library))),
                                Err(error) => {
                                    self.result = format!("{:?}", error);
                                }
                            }
                        });
                    }
                }
                if !self.result.is_empty() {
                    ui.colored_label(Color32::from_rgb(255, 0, 0), &self.result);
                }
            });
    }
}
