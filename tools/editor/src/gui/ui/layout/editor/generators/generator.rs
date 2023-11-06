use ligen_traits::generator::file_generator::FileGenerator;

use crate::prelude::*;
use crate::gui::ui::editor::{widget::Widget, settings::Settings};

pub struct Generator {
    generator: Box<dyn FileGenerator>
}

impl Generator {
    pub fn new<T: FileGenerator + 'static>(generator: T) -> Self {
        let generator = Box::new(generator);
        Self { generator }
    }
}

impl Widget for Generator {
    type Input = ligen_ir::Library;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, input: &mut Self::Input) {
        ui.label("Generator");
        if ui.button("Generate").clicked() {

        }
    }
}
