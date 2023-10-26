pub use crate::prelude::*;

mod primitive;
mod reference;
mod generics;
mod type_definition;

pub use type_definition::*;
pub use primitive::*;
pub use reference::*;
pub use generics::*;

use crate::gui::ui::editor::{ir::Path, widget::Widget, settings::Settings};

pub struct Type {
    enabled: bool
}

impl Type {
    pub fn new() -> Self {
        let enabled = true;
        Self { enabled }
    }

    pub fn enabled(&mut self, enabled: bool) -> &mut Self {
        self.enabled = enabled;
        self
    }
}

impl Widget for Type {
    type Input = ligen_ir::Type;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, type_: &mut ligen_ir::Type) {
        ui.add_enabled_ui(self.enabled, |ui| {
            if settings.editor.editable_fields {
                let variant_name = match type_ {
                    ligen_ir::Type::Primitive(_) => "Primitive",
                    ligen_ir::Type::Composite(_, _) => "Composite",
                    ligen_ir::Type::Reference(_) => "Reference",
                };
                ui.horizontal_top(|ui| {
                    egui::ComboBox::new("combo", "")
                        .selected_text(variant_name)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(type_, ligen_ir::Type::Primitive(ligen_ir::Primitive::Boolean), "Primitive");
                            ui.selectable_value(type_, ligen_ir::Type::Composite(Default::default(), Default::default()), "Composite");
                            ui.selectable_value(type_, ligen_ir::Type::Reference(Default::default()), "Reference");
                        });
                });
            }
            match type_ {
                ligen_ir::Type::Primitive(primitive) => Primitive::new().show(settings, ui, primitive),
                ligen_ir::Type::Reference(reference) => Reference::new().show(settings, ui, reference),
                ligen_ir::Type::Composite(path, generics) => {
                    ui.horizontal_top(|ui| {
                        Path::new().show(settings, ui, path);
                        Generics::new().show(settings, ui, generics);
                    });
                }
            }
        });
    }
}