use egui::CollapsingHeader;
use crate::app::ui::{StringEditableField, EnumEditableField, EditableList, Attributes};

pub struct Module {}

impl Module {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, module: &mut ligen_ir::Module) {
        let module_name = module.path.to_string(".");
        CollapsingHeader::new(module_name)
            .show(ui, |ui| {
                StringEditableField::new("module.path").show(ui, &mut module.path);
                EnumEditableField::new("module.visibility").show(ui, &mut module.visibility);
                Attributes::new("module.attributes").show(ui, &mut module.attributes);
                EditableList::new("module.modules", "Add module").show(ui, &mut module.modules, |ui, module| {
                    Module::new().show(ui, module);
                });
            });
    }
}