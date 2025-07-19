pub use crate::prelude::*;

use crate::gui::ui::editor::{ir::{Attributes, Identifier, Literal, Path}, widget::Widget, settings::Settings};

#[derive(Default)]
pub struct Attribute;

impl Attribute {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Widget for Attribute {
    type Input = ligen_idl::Attribute;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, mut attribute: &mut ligen_idl::Attribute) {
        let variant_name = match attribute {
            ligen_idl::Attribute::Literal(_) => "Literal",
            ligen_idl::Attribute::Group(_) => "Group",
            ligen_idl::Attribute::Named(_) => "Named",
        };
        ui.horizontal_top(|ui| {
            egui::ComboBox::new("combo", "")
                .selected_text(variant_name)
                .show_ui(ui, |ui| {
                    ui.selectable_value(attribute, ligen_idl::Attribute::Literal(Default::default()), "Literal");
                    ui.selectable_value(attribute, ligen_idl::Attribute::Group(Default::default()), "Group");
                    ui.selectable_value(attribute, ligen_idl::Attribute::Named(Default::default()), "Named");
                });
        });

        ui.horizontal_top(|ui| {
            match &mut attribute {
                ligen_idl::Attribute::Literal(literal) => Literal::new().show(settings, ui, literal),
                ligen_idl::Attribute::Group(group) => {
                    Path::new().show(settings, ui, &mut group.path);
                    Attributes::new().show(settings, ui, &mut group.attributes);
                },
                ligen_idl::Attribute::Named(named) => {
                    Path::new().show(settings, ui, &mut named.path);
                    Literal::new().show(settings, ui, &mut named.literal);
                }
            }
        });
    }
}