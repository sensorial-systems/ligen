pub use crate::prelude::*;

use crate::gui::ui::{EnumField, editor::{widget::Widget, settings::Settings}};

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
        EnumField::new().id_source("visibility").show(settings, ui, visibility);
    }
}