use crate::app::ui::{Literal, Path, Type};

pub struct Constant {}

impl Constant {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, constant: &mut ligen_ir::Constant) {
        ui.horizontal_top(|ui| {
            Path::new().show(ui, &mut constant.path);
            Type::new().show(ui, &mut constant.type_);
            if !constant.literal.is_compatible_with(&constant.type_) {
                constant.literal = ligen_ir::Literal::default_for_type(&constant.type_);
            }
            ui.label("=");
            Literal::new().switchable(false).show(ui, &mut constant.literal);
        });
    }
}