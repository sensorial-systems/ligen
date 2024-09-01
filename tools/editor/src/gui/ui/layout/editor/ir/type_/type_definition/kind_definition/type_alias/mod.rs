pub use crate::prelude::*;


use crate::gui::ui::{EditableList, editor::{ir::{Visibility, Identifier, Attributes, Path}, widget::Widget, settings::Settings}};

#[derive(Default)]
pub struct TypeAlias;

impl TypeAlias {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Widget for TypeAlias {
    type Input = ligen_ir::TypeAlias;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, type_alias: &mut ligen_ir::TypeAlias) {
        // EditableList::new("Variants", "Add variant").show(settings, ui, &mut enumeration.variants, |ui, variant| {
        //     Variant::new().show(settings, ui, variant);
        // });
    }
}