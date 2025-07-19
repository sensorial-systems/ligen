pub use crate::prelude::*;

use crate::gui::ui::{EnumField, editor::{widget::Widget, settings::Settings}, TextPrinter, Paper};

#[derive(Default)]
pub struct Visibility;

impl Visibility {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Widget for Visibility {
    type Input = ligen_idl::Visibility;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, visibility: &mut ligen_idl::Visibility) {
        if settings.display.show_visibility {
            EnumField::new().id_source("visibility").show(settings, ui, visibility);
        }
    }
}

impl TextPrinter for Visibility {
    type Input = ligen_idl::Visibility;
    fn print(&self, settings: &Settings, text: &mut Paper, visibility: &ligen_idl::Visibility) -> &Self {
        if settings.display.show_visibility {
            text.print_word(visibility);
        }
        self
    }
}