use crate::gui::ui::{EditorMenuButton, MenuButton, Panes};
use crate::prelude::*;
pub struct Menu {
    buttons: Vec<Box<dyn MenuButton>>
}

impl Menu {
    pub fn new() -> Self {
        let buttons: Vec<Box<dyn MenuButton>> = vec![
            Box::new(EditorMenuButton)
        ];
        Self { buttons }
    }

    pub fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, panes: &mut Panes) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal_top(|ui| {
                for button in self.buttons.iter_mut() {
                    button.menu_button(ui, panes);
                }
            });
        });
    }
}