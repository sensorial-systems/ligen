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
    type Input = ligen_ir::Visibility;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, visibility: &mut ligen_ir::Visibility) {
        if settings.display.show_visibility {
            EnumField::new().id_source("visibility").show(settings, ui, visibility);
        }
    }
}

impl TextPrinter for Visibility {
    type Input = ligen_ir::Visibility;
    fn print(&self, settings: &Settings, text: &mut Paper, visibility: &ligen_ir::Visibility) -> &Self {
        if settings.display.show_visibility {
            text.print_word(visibility);
        }
        self
    }
}