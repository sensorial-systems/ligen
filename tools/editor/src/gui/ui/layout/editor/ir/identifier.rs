pub use crate::prelude::*;

use crate::gui::ui::{StringField, editor::{widget::Widget, settings::Settings}, TextPrinter, Paper};


#[derive(Default)]
pub struct Identifier {
    string_field: StringField
}

impl Identifier {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Widget for Identifier {
    type Input = ligen_idl::Identifier;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, identifier: &mut ligen_idl::Identifier) {
        self.string_field.show(settings, ui, identifier)
    }
}

impl TextPrinter for Identifier {
    type Input = ligen_idl::Identifier;
    fn print(&self, settings: &Settings, paper: &mut Paper, input: &Self::Input) -> &Self {
        paper.print_word(input);
        self
    }
}