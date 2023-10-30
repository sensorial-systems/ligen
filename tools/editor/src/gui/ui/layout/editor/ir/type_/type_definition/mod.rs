use crate::gui::ui::{editor::{widget::{Widget, WidgetFor}, settings::Settings, ir::{Visibility, Identifier, Path, Attributes}}, EditableList, TextPrinter, Paper, Printer};
pub use crate::prelude::*;

mod kind_definition;
pub use kind_definition::*;
use ligen_gui_runtime::egui::CollapsingHeader;

#[derive(Default)]
pub struct TypeDefinition;

impl TypeDefinition {
    pub fn new() -> Self {
        Default::default()
    }
}

impl WidgetFor for ligen_ir::TypeDefinition {
    type Widget = TypeDefinition;
}

impl Widget for TypeDefinition {
    type Input = ligen_ir::TypeDefinition;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, definition: &mut ligen_ir::TypeDefinition) {
        let text = Printer::new().print(|text| {
            self.print(settings, text, definition);
        });
        CollapsingHeader::new(text)
        .id_source("type_definition")
        .show(ui, |ui| {
            if settings.editor.editable_fields {
                ui.horizontal_top(|ui| {
                    Visibility::new().show(settings, ui, &mut definition.visibility);
                    KindDefinition::new().show_kind_name(settings, ui, &mut definition.definition);
                    Identifier::new().show(settings, ui, &mut definition.identifier);
                });
            }
            EditableList::new("Interfaces", "Add interface").show(settings, ui, &mut definition.interfaces, |ui, interface| {
                Path::new().show(settings, ui, interface);
            });
            Attributes::new().show(settings, ui, &mut definition.attributes);
            KindDefinition::new().show(settings, ui, &mut definition.definition);
        });
    }
}

impl TextPrinter for TypeDefinition {
    type Input = ligen_ir::TypeDefinition;
    fn print(&self, settings: &Settings, paper: &mut Paper, input: &Self::Input) -> &Self {
        Visibility::new().print(settings, paper, &input.visibility);
        KindDefinition::new().print(settings, paper, &input.definition);
        Identifier::new().print(settings, paper, &input.identifier);
        self
    }
}