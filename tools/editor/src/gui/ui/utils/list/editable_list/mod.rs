pub use crate::prelude::*;

use egui::containers::CollapsingHeader;

use super::List;

// TODO: Implement this with List.

pub struct EditableList {
    list: List,
    editable: bool,
    add_button_name: String,
}

impl EditableList {
    pub fn new(name: impl AsRef<str>, add_button_name: impl AsRef<str>) -> Self {
        let list = List::new(name);
        let editable = false;
        let add_button_name = add_button_name.as_ref().into();
        Self { list, editable, add_button_name }
    }

    pub fn editable(&mut self, editable: bool) -> &mut Self {
        self.editable = editable;
        self
    }

    pub fn show<T>(&mut self, ui: &mut egui::Ui, list: &mut Vec<T>, mut show_item: impl FnMut(&mut egui::Ui, &mut T))
    where T: Default
    {
        if !list.is_empty() || self.editable {
            let mut remove_list = Vec::new();
            self.list.show(ui, list.iter_mut().enumerate(), |ui, (index, item)| {
                ui.horizontal_top(|ui| {
                    if self.editable && ui.button("x").clicked() {
                        remove_list.push(index);
                    }
                    ui.push_id(index, |ui| {
                        show_item(ui, item);
                    });
                });
            });
            for index in remove_list.into_iter().rev() {
                list.remove(index);
            }
            if self.editable && ui.button(&self.add_button_name).clicked() {
                list.push(T::default());
            }    
        }
    }
}