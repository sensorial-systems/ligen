pub use crate::prelude::*;

mod attribute;

pub use attribute::*;
use crate::gui::ui::EditableList;

pub struct Attributes {
}

impl Attributes {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, attributes: &mut ligen_ir::Attributes) {
        EditableList::new("Attributes", "Add attribute")
            .show(ui, &mut attributes.attributes,|ui, attribute| {
                Attribute::new().show(ui, attribute);
            });

    }
}
