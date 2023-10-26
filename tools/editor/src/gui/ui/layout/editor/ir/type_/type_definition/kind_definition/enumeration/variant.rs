pub use crate::prelude::*;

use crate::gui::ui::editor::{ir::{Attributes, Identifier}, widget::Widget, settings::Settings};

#[derive(Default)]
pub struct Variant;

impl Variant {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Widget for Variant {
    type Input = ligen_ir::Variant;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, variant: &mut ligen_ir::Variant) {
        Identifier::new().show(settings, ui, &mut variant.identifier);
        Attributes::new().show(settings, ui, &mut variant.attributes);
    }
}