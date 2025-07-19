pub use crate::prelude::*;

mod attribute;

pub use attribute::*;
use crate::gui::ui::{EditableList, editor::{widget::Widget, settings::Settings}};

#[derive(Default)]
pub struct Attributes;

impl Attributes {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Widget for Attributes {
    type Input = ligen_idl::Attributes;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, attributes: &mut ligen_idl::Attributes) {
        EditableList::new("Attributes", "Add attribute")
            .show(settings, ui, &mut attributes.attributes,|ui, attribute| {
                Attribute::new().show(settings, ui, attribute);
            });
    }
}
