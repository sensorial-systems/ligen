use crate::app::ui::StringEditableField;

pub struct Path {}

impl Path {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, path: &mut ligen_ir::Path) {
        StringEditableField::new().show(ui, path)
    }
}