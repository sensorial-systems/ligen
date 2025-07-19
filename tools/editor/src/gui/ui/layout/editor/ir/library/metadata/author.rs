use crate::{prelude::*, gui::ui::{editor::{widget::{Widget, WidgetFor}, settings::Settings}, Labeled, StringField}};

#[derive(Default)]
pub struct Author {}

impl Widget for Author {
    type Input = ligen_idl::Author;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, input: &mut Self::Input) {
        Labeled::new(&input.name).show(settings, ui, |ui| {
            StringField::default().show(settings, ui, &mut input.email);
        });
    }
}

impl WidgetFor for ligen_idl::Author {
    type Widget = Author;
}