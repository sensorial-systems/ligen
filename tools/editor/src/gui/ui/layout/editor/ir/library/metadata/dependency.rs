use crate::{prelude::*, gui::ui::{editor::{widget::{Widget, WidgetFor}, settings::Settings}, Labeled, StringField}};

#[derive(Default)]
pub struct Dependency {}

impl Widget for Dependency {
    type Input = ligen_ir::Dependency;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, input: &mut Self::Input) {
        Labeled::new(&input.identifier.name).show(settings, ui, |ui| {
            StringField::default().show(settings, ui, &mut input.requirement);
        });
    }
}

impl WidgetFor for ligen_ir::Dependency {
    type Widget = Dependency;
}