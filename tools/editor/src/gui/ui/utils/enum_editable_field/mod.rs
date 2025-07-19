use crate::gui::ui::editor::settings::Settings;
pub use crate::prelude::*;

use std::fmt::Debug;
use egui::ComboBox;
use ligen_idl::prelude::*;

#[derive(Default)]
pub struct EnumField {
    id_source: String
}

impl EnumField {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn id_source(mut self, string: impl AsRef<str>) -> Self {
        self.id_source = string.as_ref().into();
        self
    }

    pub fn show<T>(&mut self, settings: &Settings, ui: &mut egui::Ui, field: &mut T)
    where T: IntoEnumIterator + Debug + PartialEq
    {
        if settings.editor.editable_fields {
            ui.horizontal_top(|ui| {
                ComboBox::new(&self.id_source, "")
                    .selected_text(format!("{field:?}"))
                    .show_ui(ui, |ui| {
                        for variant in T::iter() {
                            if ui.add(egui::SelectableLabel::new(*field == variant, format!("{variant:?}"))).clicked() {
                                *field = variant
                            }
                        }
                    });
            });
        } else {
            ui.label(format!("{field:?}"));
        }
    }
}