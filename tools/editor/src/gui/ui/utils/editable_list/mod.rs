pub use crate::prelude::*;

use egui::containers::CollapsingHeader;

pub struct EditableList {
    editable: bool,
    name: String,
    add_button_name: String,
    id_source: String
}

impl EditableList {
    pub fn new(name: impl AsRef<str>, add_button_name: impl AsRef<str>) -> Self {
        let editable = false;
        let name = name.as_ref().to_string();
        let id_source = name.clone();
        let add_button_name = add_button_name.as_ref().into();
        Self { editable, name, add_button_name, id_source }
    }

    pub fn editable(&mut self, editable: bool) -> &mut Self {
        self.editable = editable;
        self
    }

    pub fn show<T>(&mut self, ui: &mut egui::Ui, list: &mut Vec<T>, mut show_item: impl FnMut(&mut egui::Ui, &mut T))
    where T: Default
    {
        if !list.is_empty() || self.editable {
            CollapsingHeader::new(&self.name)
                .default_open(!list.is_empty())
                .id_source(&self.id_source)
                .show(ui, |ui| {
                    let mut remove_list = Vec::new();
                    for (index, item) in list.iter_mut().enumerate() {
                        ui.horizontal_top(|ui| {
                            if self.editable && ui.button("x").clicked() {
                                remove_list.push(index);
                            }
                            ui.push_id(index, |ui| {
                                show_item(ui, item);
                            });
                        });
                    }
                    for index in remove_list.into_iter().rev() {
                        list.remove(index);
                    }
                    if self.editable && ui.button(&self.add_button_name).clicked() {
                        list.push(T::default());
                    }    
                });
        }
    }
}