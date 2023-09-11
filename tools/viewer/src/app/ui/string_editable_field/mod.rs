mod string_editable;
pub use string_editable::*;

pub struct StringEditableField {
    name: String
}

impl StringEditableField {
    pub fn new(name: impl AsRef<str>) -> Self {
        let name = name.as_ref().into();
        Self { name }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, field: &mut impl StringEditable) {
        ui.horizontal(|ui| {
            ui.label(format!("{}:", self.name));
            let mut string = field.as_string();
            ui.text_edit_singleline(&mut string);
            field.update(string);
        });
    }
}