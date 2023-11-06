pub use crate::prelude::*;

use crate::gui::ui::editor::{ir::{Attributes, Identifier, Literal}, widget::Widget, settings::Settings};

#[derive(Default)]
pub struct Attribute;

impl Attribute {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Widget for Attribute {
    type Input = ligen_ir::Attribute;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, mut attribute: &mut ligen_ir::Attribute) {
        let variant_name = match attribute {
            ligen_ir::Attribute::Literal(_) => "Literal",
            ligen_ir::Attribute::Group(_) => "Group",
            ligen_ir::Attribute::Named(_) => "Named",
        };
        ui.horizontal_top(|ui| {
            egui::ComboBox::new("combo", "")
                .selected_text(variant_name)
                .show_ui(ui, |ui| {
                    ui.selectable_value(attribute, ligen_ir::Attribute::Literal(Default::default()), "Literal");
                    ui.selectable_value(attribute, ligen_ir::Attribute::Group(Default::default()), "Group");
                    ui.selectable_value(attribute, ligen_ir::Attribute::Named(Default::default()), "Named");
                });
        });

        ui.horizontal_top(|ui| {
            match &mut attribute {
                ligen_ir::Attribute::Literal(literal) => Literal::new().show(settings, ui, literal),
                ligen_ir::Attribute::Group(group) => {
                    Identifier::new().show(settings, ui, &mut group.identifier);
                    Attributes::new().show(settings, ui, &mut group.attributes);
                },
                ligen_ir::Attribute::Named(named) => {
                    Identifier::new().show(settings, ui, &mut named.identifier);
                    Literal::new().show(settings, ui, &mut named.literal);
                }
            }
        });
    }
}