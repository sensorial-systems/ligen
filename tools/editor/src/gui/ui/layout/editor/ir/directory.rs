pub use crate::prelude::*;

use std::path::PathBuf;
use crate::gui::ui::{StringField, editor::{widget::Widget, settings::Settings}};

#[derive(Default)]
pub struct Directory;

impl Directory {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Widget for Directory {
    type Input = PathBuf;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, directory: &mut PathBuf) {
        StringField::new().show(settings, ui, directory)
    }
}