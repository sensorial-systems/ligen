mod primitive;
mod reference;
mod generics;

pub use primitive::*;
pub use reference::*;
pub use generics::*;

use crate::app::ui::Path;

pub struct Type {
}

impl Type {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, type_: &mut ligen_ir::Type) {
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
        match type_ {
            ligen_ir::Type::Primitive(primitive) => Primitive::new().show(ui, primitive),
            ligen_ir::Type::Reference(reference) => Reference::new().show(ui, reference),
            ligen_ir::Type::Composite(path, generics) => {
                ui.horizontal_top(|ui| {
                    Path::new().show(ui, path);
                    Generics::new().show(ui, generics);
                });
            }
        }
    }
}
