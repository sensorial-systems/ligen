mod generics;

use crate::gui::ui::editor::{widget::Widget, ir::Path};

pub use generics::*;

#[derive(Default)]
pub struct Composite {}

impl Composite {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Widget for Composite {
    type Input = ligen_ir::Composite;
    fn show(&mut self, settings: &crate::gui::ui::editor::settings::Settings, ui: &mut ligen_gui_runtime::egui::Ui, input: &mut Self::Input) {
        if settings.editor.editable_fields {
            ui.horizontal_top(|ui| {
                Path::new().show(settings, ui, &mut input.path);
                Generics::new().show(settings, ui, &mut input.generics);
            });
        } else {
            ui.label(input.to_string());
        }
    }
}