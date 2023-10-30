pub use crate::prelude::*;

use crate::gui::ui::{EnumField, editor::{widget::Widget, settings::Settings}, TextPrinter};

#[derive(Default)]
pub struct Synchrony;

impl Synchrony {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Widget for Synchrony {
    type Input = ligen_ir::Synchrony;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, synchrony: &mut ligen_ir::Synchrony) {
        EnumField::new().id_source("synchrony").show(settings, ui, synchrony);
    }
}

impl TextPrinter for Synchrony {
    type Input = ligen_ir::Synchrony;
    fn print(&self, settings: &Settings, paper: &mut crate::gui::ui::Paper, input: &Self::Input) -> &Self {
        paper.print_word(input);
        self
    }
}