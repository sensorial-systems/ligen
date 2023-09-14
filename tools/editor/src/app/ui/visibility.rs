use crate::app::ui::EnumEditableField;

pub struct Visibility {
}

impl Visibility {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, visibility: &mut ligen_ir::Visibility) {
        EnumEditableField::new().id_source("visibility").show(ui, visibility);
    }
}