pub use crate::prelude::*;

use crate::gui::ui::{Attributes, Identifier};

pub struct Variant {}

impl Variant {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, variant: &mut ligen_ir::Variant) {
        Identifier::new().show(ui, &mut variant.identifier);
        Attributes::new().show(ui, &mut variant.attributes);
    }
}