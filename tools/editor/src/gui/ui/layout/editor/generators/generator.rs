use ligen_generator::prelude::*;

use crate::prelude::*;
use crate::gui::ui::editor::{widget::Widget, settings::Settings};

pub struct Generator {
    generator: Box<dyn ligen_generator::Generator<ligen_ir::Library, Output = ()>>,
    result: String
}

impl Generator {
    pub fn new<T: ligen_generator::Generator<ligen_ir::Library, Output = ()> + 'static>(generator: T) -> Self {
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
            let entry = rfd::FileDialog::new()
            .pick_folder();
            if let Some(entry) = entry {
                let mut config = Config::default();
                config.set("path", entry.to_string_lossy().to_string());
                match self.generator.generate(input, &config) {
                    Ok(_) => self.result = "Success".to_string(),
                    Err(error) => self.result = format!("Error: {:?}", error)
                };
            }
        }
        ui.label(&self.result);
    }
}
