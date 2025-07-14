use crate::gui::ui::editor::{widget::Widget, settings::Settings};
pub use crate::prelude::*;

use egui::ComboBox;

#[derive(Default)]
pub struct Literal {}

impl Literal {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Widget for Literal {
    type Input = ligen_ir::Literal;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, mut literal: &mut ligen_ir::Literal) {
        if settings.editor.editable_fields {
            let variant_name = match literal {
                ligen_ir::Literal::String(_) => "String",
                ligen_ir::Literal::Boolean(_) => "Bool",
                ligen_ir::Literal::Integer(_) => "Integer",
                ligen_ir::Literal::Float(_) => "Float",
                ligen_ir::Literal::Character(_) => "Char",
                ligen_ir::Literal::UnsignedInteger(_) => "Unsigned Integer",
                ligen_ir::Literal::None => "None",
                ligen_ir::Literal::Unknown(_) => "Unknown",
                ligen_ir::Literal::Tuple(_) => "Tuple",
                ligen_ir::Literal::Vector(_) => "Vector",
            };
            ui.horizontal_top(|ui| {
                ComboBox::new("Literal", "")
                    .selected_text(variant_name)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(literal, ligen_ir::Literal::Boolean(false), "Bool");
                        ui.selectable_value(literal, ligen_ir::Literal::String(Default::default()), "String");
                        ui.selectable_value(literal, ligen_ir::Literal::Integer(0), "Integer");
                        ui.selectable_value(literal, ligen_ir::Literal::Float(0.0), "Float");
                        ui.selectable_value(literal, ligen_ir::Literal::Character('A'), "Char");
                        ui.selectable_value(literal, ligen_ir::Literal::UnsignedInteger(0), "Unsigned Integer");
                    });
                match &mut literal {
                    ligen_ir::Literal::Boolean(value) => {
                        if ui.button(value.to_string()).clicked() {
                            *value = !*value;
                        }
                    },
                    ligen_ir::Literal::String(value) => {
                        ui.text_edit_singleline(value);
                    },
                    ligen_ir::Literal::Character(value) => {
                        let mut string = format!("{value}");
                        ui.text_edit_singleline(&mut string);
                        *value = string.chars().next().unwrap_or(' ');
                    },
                    ligen_ir::Literal::Integer(value) => {
                        ui.add(egui::DragValue::new(value));
                    },
                    ligen_ir::Literal::UnsignedInteger(value) => {
                        ui.add(egui::DragValue::new(value));
                    },
                    ligen_ir::Literal::Float(value) => {
                        ui.add(egui::DragValue::new(value));
                    },
                    ligen_ir::Literal::None => {
                        ui.label("None");
                    },
                    ligen_ir::Literal::Unknown(_) => {
                        ui.label("Unknown");
                    },
                    // TODO: Tuple and Vector
                    ligen_ir::Literal::Tuple(value) => {
                        ui.label(format!("{value:?}"));
                    },
                    ligen_ir::Literal::Vector(value) => {
                        ui.label(format!("{value:?}"));
                    },
                }
            });
        } else {
            ui.label(literal.to_string());
        }
    }
}