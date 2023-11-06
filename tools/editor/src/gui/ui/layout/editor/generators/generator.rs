use ligen_cargo::parser::library;
use ligen_traits::generator;

use crate::prelude::*;
use crate::gui::ui::editor::{widget::Widget, settings::Settings};

pub struct Generator {
    generator: Box<dyn generator::Generator>,
    result: String
}

impl Generator {
    pub fn new<T: generator::Generator + 'static>(generator: T) -> Self {
        let generator = Box::new(generator);
        let result = Default::default();
        Self { generator, result }
    }
}

impl Widget for Generator {
    type Input = ligen_ir::Library;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, input: &mut Self::Input) {
        ui.label("Generator");
        if ui.button("Generate").clicked() {
            match self.generator.generate(input) {
                Ok(_) => self.result = "Success".to_string(),
                Err(error) => self.result = format!("Error: {:?}", error)
            };
        }
        ui.label(&self.result);
    }
}
