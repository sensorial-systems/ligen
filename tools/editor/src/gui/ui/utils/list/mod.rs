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

    pub fn show<'a, T: 'a>(&mut self, ui: &mut egui::Ui, list: impl IntoIterator<Item = &'a mut T>, mut show_item: impl FnMut(&mut egui::Ui, &mut T))
    where T: Default
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