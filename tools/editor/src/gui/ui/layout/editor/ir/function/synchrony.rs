pub use crate::prelude::*;

use crate::gui::ui::{EnumField, editor::{widget::Widget, settings::Settings}};

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