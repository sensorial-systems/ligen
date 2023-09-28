pub use crate::prelude::*;

use crate::ligen_editor::ui::{Attributes, Identifier, Literal};

pub struct Attribute {

}

impl Attribute {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, mut attribute: &mut ligen_ir::Attribute) {
        let variant_name = match attribute {
            ligen_ir::Attribute::Literal(_) => "Literal",
            ligen_ir::Attribute::Group(_, _) => "Group",
            ligen_ir::Attribute::Named(_, _) => "Named",
        };
        ui.horizontal_top(|ui| {
            egui::ComboBox::new("combo", "")
                .selected_text(variant_name)
                .show_ui(ui, |ui| {
                    ui.selectable_value(attribute, ligen_ir::Attribute::Literal(Default::default()), "Literal");
                    ui.selectable_value(attribute, ligen_ir::Attribute::Group(Default::default(), Default::default()), "Group");
                    ui.selectable_value(attribute, ligen_ir::Attribute::Named(Default::default(), Default::default()), "Named");
                });
        });

        ui.horizontal_top(|ui| {
            match &mut attribute {
                ligen_ir::Attribute::Literal(literal) => Literal::new().show(ui, literal),
                ligen_ir::Attribute::Group(identifier, attributes) => {
                    Identifier::new().show(ui, identifier);
                    Attributes::new().show(ui, attributes);
                },
                ligen_ir::Attribute::Named(identifier, literal) => {
                    Identifier::new().show(ui, identifier);
                    Literal::new().show(ui, literal);
                }
            }
        });
    }
}