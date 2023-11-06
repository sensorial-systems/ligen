pub use crate::prelude::*;

use crate::gui::ui::{StringField, editor::{widget::{Widget, WidgetFor}, settings::Settings}};

#[derive(Default)]
pub struct Path {}

impl Path {
    pub fn new() -> Self {
        Default::default()
    }
}

impl WidgetFor for ligen_ir::Path {
    type Widget = Path;
}

impl Widget for Path {
    type Input = ligen_ir::Path;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, path: &mut ligen_ir::Path) {
        StringField::new().show(settings, ui, path)
    }
}