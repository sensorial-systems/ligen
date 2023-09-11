use std::fmt::Debug;
use egui::ComboBox;
use ligen_ir::prelude::*;

pub struct EnumEditableField {
    name: String
}

impl EnumEditableField {
    pub fn new(name: impl AsRef<str>) -> Self {
        let name = name.as_ref().into();
        Self { name }
    }

    pub fn show<T>(&mut self, ui: &mut egui::Ui, field: &mut T)
    where T: IntoEnumIterator + Debug + PartialEq
    {
        ui.horizontal(|ui| {
            ui.label(format!("{}:", self.name));
            ComboBox::new("combo", "")
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