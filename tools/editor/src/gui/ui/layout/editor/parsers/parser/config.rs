use crate::{prelude::*, gui::ui::editor::ir::{Path, Literal}};
use ligen_parser::prelude::*;

use crate::gui::ui::editor::{widget::Widget, settings::Settings};

// TODO: ParserConfig was renamed to Config which is now usable in Transformer, Generator, Parser and Validator.
#[derive(Default)]
pub struct ParserConfig {}

impl ParserConfig {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Widget for ParserConfig {
    type Input = ligen_parser::prelude::Config;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, input: &mut Self::Input) {
        ui.label("Configuration");
        for (index, (mut path, mut literal)) in input.clone().iter().enumerate() {
            ui.push_id(index, |ui| {
                ui.horizontal(|ui| {
                    Path::default().show(settings, ui, &mut path);
                    Literal::default().show(settings, ui, &mut literal);
                    input.set(path, literal);
                });
            });
        }
    }
}