use crate::prelude::*;
use super::settings::Settings;

pub trait Widget {
    type Input;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, input: &mut Self::Input);
}
