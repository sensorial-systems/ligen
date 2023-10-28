use crate::gui::ui::editor::settings::Settings;
use crate::gui::ui::editor::widget::{Widget, WidgetFor};
pub use crate::prelude::*;

use egui::CollapsingHeader;
use crate::gui::ui::{EditableList, TextPrinter, Printer, Paper, SubWidgets, SubWidgetsWithSymbols, SymbolsCount};
use crate::gui::ui::editor::ir::{Attributes, Import, Visibility, Object, Function, Identifier, Type, TypeDefinition, Interface};

#[derive(Default)]
pub struct Module;

impl Module {
    pub fn new() -> Self {
        Default::default()
    }
}

impl WidgetFor for ligen_ir::Module {
    type Widget = Module;
}

impl Widget for Module {
    type Input = ligen_ir::Module;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, module: &mut ligen_ir::Module) {
        let text = Printer::new().print(|text| {
            self.print(settings, text, module);
        });
        CollapsingHeader::new(text)
            .id_source("module")
            .show(ui, |ui| {
                if settings.editor.editable_fields {
                    ui.horizontal_top(|ui| {
                        Visibility::new().show(settings, ui, &mut module.visibility);
                        Identifier::new().show(settings, ui, &mut module.identifier);
                    });
                }
                SubWidgetsWithSymbols::new("Type").show(settings, ui, &mut module.types);
                SubWidgetsWithSymbols::new("Object").show(settings, ui, &mut module.objects);
                SubWidgetsWithSymbols::new("Function").show(settings, ui, &mut module.functions);
                SubWidgetsWithSymbols::new("Interface").show(settings, ui, &mut module.interfaces);
                SubWidgetsWithSymbols::new("Module").show(settings, ui, &mut module.modules);
                SubWidgets::new("Import").show(settings, ui, &mut module.imports);
                Attributes::new().show(settings, ui, &mut module.attributes);
            });
    }
}

impl TextPrinter for Module {
    type Input = ligen_ir::Module;
    fn print(&self, settings: &Settings, paper: &mut Paper, input: &Self::Input) -> &Self {
        Visibility::new().print(settings, paper, &input.visibility);
        Identifier::new().print(settings, paper, &input.identifier);
        SymbolsCount::new().print(settings, paper, input);
        self
    }
}