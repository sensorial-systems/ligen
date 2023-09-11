use crate::app::ui::{Attributes, Literal, StringEditableField};

pub struct Attribute {

}

impl Attribute {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, mut attribute: &mut ligen_ir::Attribute) {
        ui.horizontal(|ui| {
            match &mut attribute {
                ligen_ir::Attribute::Literal(literal) => Literal::new().show(ui, literal),
                ligen_ir::Attribute::Group(identifier, attributes) => {
                    StringEditableField::new("identifier").show(ui, identifier);
                    Attributes::new("attributes").show(ui, attributes);
                },
                ligen_ir::Attribute::Named(identifier, literal) => {
                    StringEditableField::new("identifier").show(ui, identifier);
                    Literal::new().show(ui, literal);
                }
            }
        });
    }
}