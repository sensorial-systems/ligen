use std::fmt::Debug;
use egui::ComboBox;
use ligen_ir::prelude::*;

pub struct EnumEditableField {
    id_source: String,
}

impl EnumEditableField {
    pub fn new() -> Self {
        let id_source = Default::default();
        Self { id_source }
    }

    pub fn id_source(mut self, string: impl AsRef<str>) -> Self {
        self.id_source = string.as_ref().into();
        self
    }

    pub fn show<T>(&mut self, ui: &mut egui::Ui, field: &mut T)
    where T: IntoEnumIterator + Debug + PartialEq
    {
        ui.horizontal_top(|ui| {
            ComboBox::new(&self.id_source, "")
                .selected_text(format!("{:?}", field))
                .show_ui(ui, |ui| {
                    for variant in T::iter() {
                        if ui.add(egui::SelectableLabel::new(*field == variant, format!("{:?}", variant))).clicked() {
                            *field = variant
                        }
                    }
                });
        });
    }
}