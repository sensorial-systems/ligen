use egui::CollapsingHeader;
use crate::app::ui::{EditableList, Attributes, Import, Path, Visibility, Constant, Function};

pub struct Module {}

impl Module {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, module: &mut ligen_ir::Module) {
        let module_name = module.path.to_string(".");
        CollapsingHeader::new(module_name)
            .id_source("module")
            .show(ui, |ui| {
                ui.horizontal_top(|ui| {
                    Visibility::new().show(ui, &mut module.visibility);
                    Path::new().show(ui, &mut module.path);
                });
                EditableList::new("Imports", "Add import").show(ui, &mut module.imports, |ui, import| {
                    Import::new().show(ui, import);
                });
                EditableList::new("Constants", "Add constant").show(ui, &mut module.constants, |ui, constant| {
                    Constant::new().show(ui, constant);
                });
                EditableList::new("Functions", "Add function").show(ui, &mut module.functions, |ui, function| {
                    Function::new().show(ui, function);
                });
                EditableList::new("Modules", "Add module").show(ui, &mut module.modules, |ui, module| {
                    Module::new().show(ui, module);
                });
                Attributes::new().show(ui, &mut module.attributes);
            });
    }
}