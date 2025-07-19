pub use crate::prelude::*;

mod variant;

pub use variant::*;

use crate::gui::ui::{EditableList, editor::{ir::{Visibility, Identifier, Attributes, Path}, widget::Widget, settings::Settings}};

#[derive(Default)]
pub struct Enumeration;

impl Enumeration {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Widget for Enumeration {
    type Input = ligen_idl::Enumeration;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, enumeration: &mut ligen_idl::Enumeration) {
        EditableList::new("Variants", "Add variant").show(settings, ui, &mut enumeration.variants, |ui, variant| {
            Variant::new().show(settings, ui, variant);
        });
    }
}