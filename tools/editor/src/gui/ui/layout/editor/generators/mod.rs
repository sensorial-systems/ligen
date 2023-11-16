pub mod generator;
pub use generator::*;
use ligen_rust_pyo3_importer::LibraryGenerator;

use crate::gui::ui::panes::{Pane, PaneManager};

use super::{settings::Settings, widget::Widget};

pub struct Generators {
    generators: Vec<Generator>
}

impl Default for Generators {
    fn default() -> Self {
        let generators = vec![
            Generator::new(LibraryGenerator::default())
        ];
        Self { generators }
    }
}

impl Generators {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Widget for Generators {
    type Input = ligen_ir::Library;
    fn show(&mut self, settings: &Settings, ui: &mut ligen_gui_runtime::egui::Ui, input: &mut Self::Input) {
        for (index, generator) in self.generators.iter_mut().enumerate() {
            ui.push_id(index, |ui| {
                generator.show(settings, ui, input);
            });
        }
    }
}