use crate::prelude::*;

pub struct DisplaySettings {
    pub show_attributes: bool,
    pub show_visibility: bool,
    pub show_type: bool,
    pub show_path: bool,
    pub show_import: bool,
    pub show_literal: bool,
    pub show_directory: bool,
    pub show_menu_button: bool,
    pub show_interface: bool,
    pub show_object: bool,
    pub show_function: bool,
    pub show_identifier: bool,
    pub show_module: bool,
    pub show_project: bool,
    pub show_symbols_count: bool,
}

impl Default for DisplaySettings {
    fn default() -> Self {
        Self {
            show_attributes: true,
            show_visibility: true,
            show_type: true,
            show_path: true,
            show_import: true,
            show_literal: true,
            show_directory: true,
            show_menu_button: true,
            show_interface: true,
            show_object: true,
            show_function: true,
            show_identifier: true,
            show_module: true,
            show_project: true,
            show_symbols_count: true,
        }
    }
}

impl DisplaySettings {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.label("Display Settings");
        ui.horizontal_wrapped(|ui| {
            ui.checkbox(&mut self.show_attributes, "Attributes");
            ui.checkbox(&mut self.show_visibility, "Visibility");
            ui.checkbox(&mut self.show_type, "Type");
            ui.checkbox(&mut self.show_path, "Path");
            ui.checkbox(&mut self.show_import, "Import");
            ui.checkbox(&mut self.show_literal, "Literal");
            ui.checkbox(&mut self.show_directory, "Directory");
            ui.checkbox(&mut self.show_menu_button, "Menu Button");
            ui.checkbox(&mut self.show_interface, "Interface");
            ui.checkbox(&mut self.show_object, "Object");
            ui.checkbox(&mut self.show_function, "Function");
            ui.checkbox(&mut self.show_identifier, "Identifier");
            ui.checkbox(&mut self.show_module, "Module");
            ui.checkbox(&mut self.show_project, "Project");
            ui.checkbox(&mut self.show_symbols_count, "Symbols Count");
        });
    }
}