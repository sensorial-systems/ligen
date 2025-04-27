use ligen_generator::prelude::*;

use crate::prelude::*;
use crate::gui::ui::editor::{widget::Widget, settings::Settings};

pub struct Generator {
    generator: Box<dyn for<'a> ligen_generator::Generator<&'a ligen_ir::Library, ()>>,
    result: String
}

impl Generator {
    pub fn new<T>(generator: T) -> Self
    where T: for<'a> ligen_generator::Generator<&'a ligen_ir::Library, ()> + 'static
    {
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
