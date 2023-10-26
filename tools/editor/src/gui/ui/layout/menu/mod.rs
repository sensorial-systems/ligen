use crate::gui::ui::editor::ir;
use crate::gui::ui::panes::Panes;
use crate::prelude::*;

pub trait MenuButton {
    fn menu_title(&self) -> String;
    fn show_button(&self, ui: &mut egui::Ui, panes: &mut Panes);

    fn menu_button(&self, ui: &mut egui::Ui, panes: &mut Panes) {
        ui.menu_button(self.menu_title(), |ui| {
            self.show_button(ui, panes)
        });
    }
}

pub struct Menu {
    buttons: Vec<Box<dyn MenuButton>>
}

impl Menu {
    pub fn new() -> Self {
        let buttons: Vec<Box<dyn MenuButton>> = vec![
            Box::new(ir::EditorMenuButton)
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