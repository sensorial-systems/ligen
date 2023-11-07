mod sub_widgets_with_symbols;

use crate::gui::ui::editor::widget::{WidgetFor, Widget};
use crate::prelude::*;
use crate::gui::ui::editor::settings::Settings;
use super::{EditableList, Printer};

pub use sub_widgets_with_symbols::*;

// TODO: Find a better name.
pub struct SubWidgets {
    plural: String,
    add_button_name: String
}

impl SubWidgets {
    pub fn new(name: impl AsRef<str>) -> Self {
        let singular = name.as_ref().to_string();
        let plural = singular.clone() + "s";
        Self::new_irregular(singular, plural)
    }

    pub fn new_irregular(singular: impl AsRef<str>, plural: impl AsRef<str>) -> Self {
        let singular = singular.as_ref().to_string();
        let plural = plural.as_ref().to_string();
        let add_button_name = format!("Add {}", singular.to_lowercase());
        Self { plural, add_button_name }
    }

    fn show_list<T: WidgetFor>(&mut self, name: impl AsRef<str>, settings: &Settings, ui: &mut egui::Ui, input: &mut Vec<T>) {
        EditableList::new(name, &self.add_button_name)
            .id_source(&self.plural)
            .show(settings, ui, input, |ui, type_| {
                <T as WidgetFor>::Widget::default().show(settings, ui, type_);
            });
    }
}

impl SubWidgets {
    pub fn show<T: WidgetFor>(&mut self, settings: &Settings, ui: &mut egui::Ui, input: &mut Vec<T>)
    {
        self.show_list(self.plural.clone(), settings, ui, input);
    }
}
