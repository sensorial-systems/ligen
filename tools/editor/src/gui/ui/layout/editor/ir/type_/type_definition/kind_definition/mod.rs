use ligen_gui_runtime::egui::ComboBox;

mod structure;
mod enumeration;
mod type_alias;

pub use structure::*;
pub use enumeration::*;
pub use type_alias::*;
use crate::gui::ui::TextPrinter;
use crate::gui::ui::editor::settings::Settings;
use crate::prelude::*;
use crate::gui::ui::editor::widget::Widget;



#[derive(Default)]
pub struct KindDefinition;

impl KindDefinition {
    pub fn new() -> Self {
        Default::default()
    }
}

impl KindDefinition {
    /// Show the kind definition.
    pub fn show_kind_name(&mut self, settings: &Settings, ui: &mut egui::Ui, definition: &mut ligen_ir::KindDefinition) {
        let variant_name = definition.kind_name();
        if settings.editor.editable_fields {
            ComboBox::new("Kind", "")
                .selected_text(variant_name)
                .show_ui(ui, |ui| {
                    ui.selectable_value(definition, ligen_ir::KindDefinition::Structure(Default::default()), "Structure");
                    ui.selectable_value(definition, ligen_ir::KindDefinition::Enumeration(Default::default()), "Enumeration");
                });
        } else {
            ui.label(variant_name);
        }
    }
}

impl Widget for KindDefinition {
    type Input = ligen_ir::KindDefinition;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, definition: &mut Self::Input) {
        match definition {
            ligen_ir::KindDefinition::Structure(structure) => Structure::new().show(settings, ui, structure),
            ligen_ir::KindDefinition::Enumeration(enumeration) => Enumeration::new().show(settings, ui, enumeration),
            ligen_ir::KindDefinition::TypeAlias(type_alias) => TypeAlias::new().show(settings, ui, type_alias),
        }
    }
}

impl TextPrinter for KindDefinition {
    type Input = ligen_ir::KindDefinition;
    fn print(&self, settings: &Settings, paper: &mut crate::gui::ui::Paper, input: &Self::Input) -> &Self {
        paper.print_word(input.kind_name());
        self
    }
}