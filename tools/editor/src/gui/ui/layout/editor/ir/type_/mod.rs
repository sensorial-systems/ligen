pub use crate::prelude::*;

mod reference;
mod type_definition;

pub use type_definition::*;
pub use reference::*;

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
                    ligen_ir::Type::Path(_) => "Path",
                    ligen_ir::Type::Reference(_) => "Reference",
                };
                ui.horizontal_top(|ui| {
                    egui::ComboBox::new("combo", "")
                        .selected_text(variant_name)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(type_, ligen_ir::Type::Path(Default::default()), "Path");
                            ui.selectable_value(type_, ligen_ir::Type::Reference(Default::default()), "Reference");
                        });
                });
            }
            match type_ {
                ligen_ir::Type::Path(path) => Path::new().show(settings, ui, path),
                ligen_ir::Type::Reference(reference) => Reference::new().show(settings, ui, reference),
            }
        });
    }
}