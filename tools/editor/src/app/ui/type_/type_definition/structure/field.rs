use crate::app::ui::{Attributes, Identifier, OptionalField, Type};

pub struct Field {}

impl Field {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, field: &mut ligen_ir::Field) {
        OptionalField::new("Identifier").show(ui, &mut field.identifier, |ui, identifier| {
            Identifier::new().show(ui, identifier);
        });
        Type::new().show(ui, &mut field.type_);
        Attributes::new().show(ui, &mut field.attributes);
    }
}