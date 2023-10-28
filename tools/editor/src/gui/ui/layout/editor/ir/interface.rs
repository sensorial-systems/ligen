use ligen_gui_runtime::egui::CollapsingHeader;

use crate::{prelude::*, gui::ui::{EditableList, editor::{widget::{Widget, WidgetFor}, settings::Settings}, SubWidgetsWithSymbols, SubWidgets, Printer, TextPrinter, Paper, SymbolsCount}};

use super::{Attributes, Method, Function, Object, Path, Identifier, Visibility};

#[derive(Default)]
pub struct Interface;

impl Interface {
    pub fn new() -> Self {
        Default::default()
    }
}

impl WidgetFor for ligen_ir::Interface {
    type Widget = Interface;
}

impl Widget for Interface {
    type Input = ligen_ir::Interface;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, interface: &mut ligen_ir::Interface) {
        let text = Printer::new().print(|text| {
            self.print(settings, text, interface);
        });
        CollapsingHeader::new(text)
            .id_source("module")
            .show(ui, |ui| {
                if settings.editor.editable_fields {
                    ui.horizontal_top(|ui| {
                        Visibility::new().show(settings, ui, &mut interface.visibility);
                        Identifier::new().show(settings, ui, &mut interface.identifier);
                    });
                }
                SubWidgetsWithSymbols::new("Object").show(settings, ui, &mut interface.objects);
                SubWidgetsWithSymbols::new("Function").show(settings, ui, &mut interface.functions);
                SubWidgetsWithSymbols::new("Method").show(settings, ui, &mut interface.methods);
                SubWidgets::new("Interface").show(settings, ui, &mut interface.interfaces);
                Attributes::new().show(settings, ui, &mut interface.attributes);        
            });
    }
}

impl TextPrinter for Interface {
    type Input = ligen_ir::Interface;
    fn print(&self, settings: &Settings, paper: &mut Paper, input: &ligen_ir::Interface) -> &Self {
        Visibility::new().print(settings, paper, &input.visibility);
        Identifier::new().print(settings, paper, &input.identifier);
        SymbolsCount::new().print(settings, paper, &*input);
        self
    }
}