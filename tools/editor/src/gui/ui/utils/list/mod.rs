mod editable_list;

pub use editable_list::*;

pub use crate::prelude::*;

use egui::containers::CollapsingHeader;

pub struct List {
    name: String,
    id_source: String
}

impl List {
    pub fn new(name: impl AsRef<str>) -> Self {
        let name = name.as_ref().to_string();
        let id_source = name.clone();
        Self { name, id_source }
    }

    pub fn id_source(&mut self, id_source: impl Into<String>) -> &mut Self {
        self.id_source = id_source.into();
        self
    }

    pub fn show<T>(&mut self, ui: &mut egui::Ui, list: impl IntoIterator<Item = T>, mut show_item: impl FnMut(&mut egui::Ui, T))
    {
        let list = list.into_iter();
        CollapsingHeader::new(&self.name)
            .id_source(&self.id_source)
            .show(ui, |ui| {
                for (index, item) in list.enumerate() {
                    ui.horizontal_top(|ui| {
                        ui.push_id(index, |ui| {
                            show_item(ui, item);
                        });
                    });
                }
            });
    }
}