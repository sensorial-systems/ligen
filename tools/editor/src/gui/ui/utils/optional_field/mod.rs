pub use crate::prelude::*;

use egui::Button;

pub struct OptionalField {
    text: String
}

impl OptionalField {
    pub fn new(text: impl AsRef<str>) -> Self {
        let text = text.as_ref().into();
        Self { text }
    }

    pub fn show<T: Default>(&mut self, ui: &mut egui::Ui, mut optional: &mut Option<T>, mut show: impl FnMut(&mut egui::Ui, &mut T)) {
        if ui.add(Button::new(&self.text)).clicked() {
            *optional = if optional.is_some() {
                None
            } else {
                Some(Default::default())
            }
        }
        if let Some(optional) = &mut optional {
            show(ui, optional);
        }

    }
}