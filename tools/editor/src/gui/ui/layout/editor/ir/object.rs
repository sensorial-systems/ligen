// use crate::gui::ui::EditableList;
// use crate::gui::ui::editor::ir::{Attributes, Constant, Function, Identifier, Method, TypeDefinition, Visibility};
// pub use crate::prelude::*;
//
// pub struct Object {
//
// }
//
// impl Object {
//     pub fn new() -> Self {
//         Self {}
//     }
//
//     pub fn show(&mut self, ui: &mut egui::Ui, object: &mut ligen_ir::Object) {
//         ui.vertical(|ui| {
//             ui.horizontal_top(|ui| {
//                 Visibility::new().show(ui, &mut object.visibility);
//                 Identifier::new().show(ui, &mut object.identifier);
//             });
//             TypeDefinition::new().show(ui, &mut object.definition);
//             EditableList::new("Constants", "Add constant").show(ui, &mut object.constants, |ui, constant| {
//                 Constant::new().show(ui, constant);
//             });
//             EditableList::new("Functions", "Add function").show(ui, &mut object.functions, |ui, function| {
//                 Function::new().show(ui, function);
//             });
//             EditableList::new("Methods", "Add method").show(ui, &mut object.methods, |ui, method| {
//                 Method::new().show(ui, method);
//             });
//             Attributes::new().show(ui, &mut object.attributes);
//         });
//     }
// }