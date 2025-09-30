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
    type Input = ligen_idl::Literal;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, mut literal: &mut ligen_idl::Literal) {
        if settings.editor.editable_fields {
            let variant_name = match literal {
                ligen_idl::Literal::String(_) => "String",
                ligen_idl::Literal::Boolean(_) => "Bool",
                ligen_idl::Literal::Integer(_) => "Integer",
                ligen_idl::Literal::Float(_) => "Float",
                ligen_idl::Literal::Character(_) => "Char",
                ligen_idl::Literal::UnsignedInteger(_) => "Unsigned Integer",
                ligen_idl::Literal::None => "None",
                ligen_idl::Literal::Unknown(_) => "Unknown",
                ligen_idl::Literal::Tuple(_) => "Tuple",
                ligen_idl::Literal::Array(_) => "Array",
            };
            ui.horizontal_top(|ui| {
                ComboBox::new("Literal", "")
                    .selected_text(variant_name)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(literal, ligen_idl::Literal::Boolean(false), "Bool");
                        ui.selectable_value(literal, ligen_idl::Literal::String(Default::default()), "String");
                        ui.selectable_value(literal, ligen_idl::Literal::Integer(0), "Integer");
                        ui.selectable_value(literal, ligen_idl::Literal::Float(0.0), "Float");
                        ui.selectable_value(literal, ligen_idl::Literal::Character('A'), "Char");
                        ui.selectable_value(literal, ligen_idl::Literal::UnsignedInteger(0), "Unsigned Integer");
                    });
                match &mut literal {
                    ligen_idl::Literal::Boolean(value) => {
                        if ui.button(value.to_string()).clicked() {
                            *value = !*value;
                        }
                    },
                    ligen_idl::Literal::String(value) => {
                        ui.text_edit_singleline(value);
                    },
                    ligen_idl::Literal::Character(value) => {
                        let mut string = format!("{value}");
                        ui.text_edit_singleline(&mut string);
                        *value = string.chars().next().unwrap_or(' ');
                    },
                    ligen_idl::Literal::Integer(value) => {
                        ui.add(egui::DragValue::new(value));
                    },
                    ligen_idl::Literal::UnsignedInteger(value) => {
                        ui.add(egui::DragValue::new(value));
                    },
                    ligen_idl::Literal::Float(value) => {
                        ui.add(egui::DragValue::new(value));
                    },
                    ligen_idl::Literal::None => {
                        ui.label("None");
                    },
                    ligen_idl::Literal::Unknown(_) => {
                        ui.label("Unknown");
                    },
                    // TODO: Tuple and Vector
                    ligen_idl::Literal::Tuple(value) => {
                        ui.label(format!("{value:?}"));
                    },
                    ligen_idl::Literal::Array(value) => {
                        ui.label(format!("{value:?}"));
                    },
                }
            });
        } else {
            ui.label(literal.to_string());
        }
    }
}