use crate::gui::ui::editor::settings::Settings;
use crate::gui::ui::editor::widget::{Widget, WidgetFor};
pub use crate::prelude::*;

mod synchrony;
mod parameter;
mod method;

pub use method::*;
pub use synchrony::*;
pub use parameter::*;

use crate::gui::ui::{OptionalField, EditableList};
use crate::gui::ui::editor::ir::{Attributes, Identifier, Type, Visibility};

#[derive(Default)]
pub struct Function;

impl Function {
    pub fn new() -> Self {
        Default::default()
    }
}

impl WidgetFor for ligen_ir::Function {
    type Widget = Function;
}

impl Widget for Function {
    type Input = ligen_ir::Function;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, function: &mut ligen_ir::Function) {
        ui.horizontal_top(|ui| {
            Visibility::new().show(settings, ui, &mut function.visibility);
            Synchrony::new().show(settings, ui, &mut function.synchrony);
            Identifier::new().show(settings, ui, &mut function.identifier);
            EditableList::new("Inputs", "Add input").show(settings, ui, &mut function.inputs, |ui, parameter| {
                Parameter::new().show(settings, ui, parameter);
            });
            OptionalField::new("Output").show(settings, ui, &mut function.output, |ui, output| {
                Type::new().show(settings, ui, output);
            });
            Attributes::new().show(settings, ui, &mut function.attributes);
        });
    }
}