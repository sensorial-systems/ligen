use crate::gui::ui::editor::settings::Settings;
use crate::gui::ui::editor::widget::{Widget, WidgetFor};
use crate::gui::ui::{EditableList, OptionalField};
use crate::gui::ui::editor::ir::{Attributes, Identifier, Parameter, Synchrony, Type, Visibility};
pub use crate::prelude::*;

#[derive(Default)]
pub struct Method;

impl Method {
    pub fn new() -> Self {
        Default::default()
    }
}

impl WidgetFor for ligen_ir::Method {
    type Widget = Method;
}

impl Widget for Method {
    type Input = ligen_ir::Method;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, method: &mut ligen_ir::Method) {
        ui.horizontal_top(|ui| {
            Visibility::new().show(settings, ui, &mut method.visibility);
            Synchrony::new().show(settings, ui, &mut method.synchrony);
            Identifier::new().show(settings, ui, &mut method.identifier);
            EditableList::new("Inputs", "Add input").show(settings, ui, &mut method.inputs, |ui, parameter| {
                Parameter::new().show(settings, ui, parameter);
            });
            OptionalField::new("Output").show(settings, ui, &mut method.output, |ui, output| {
                Type::new().show(settings, ui, output);
            });
            Attributes::new().show(settings, ui, &mut method.attributes);
        });
    }
}