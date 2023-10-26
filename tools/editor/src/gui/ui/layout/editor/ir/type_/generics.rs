use crate::gui::ui::editor::settings::Settings;
use crate::gui::ui::editor::widget::Widget;
pub use crate::prelude::*;

use crate::gui::ui::EditableList;
use crate::gui::ui::editor::ir::Type;

#[derive(Default)]
pub struct Generics;

impl Generics {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Widget for Generics {
    type Input = ligen_ir::Generics;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, generics: &mut ligen_ir::Generics) {
        EditableList::new(generics.to_string(), "Add type").show(settings, ui, &mut generics.types, |ui, type_| {
            Type::new().show(settings, ui, type_);
        });
    }
}