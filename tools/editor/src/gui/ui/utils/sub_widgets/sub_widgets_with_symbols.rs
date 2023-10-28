use crate::gui::ui::{Printer, SymbolsCount};
use crate::gui::ui::editor::widget::WidgetFor;
use crate::prelude::*;
use crate::gui::ui::editor::settings::Settings;

use super::SubWidgets;

pub struct SubWidgetsWithSymbols {
    sub_widgets: SubWidgets
}

impl SubWidgetsWithSymbols {
    pub fn new(name: impl AsRef<str>) -> Self {
        let sub_widgets = SubWidgets::new(name);
        Self { sub_widgets }
    }

    pub fn show<T: WidgetFor>(&mut self, settings: &Settings, ui: &mut egui::Ui, input: &mut Vec<T>)
    where for<'a> &'a Vec<T>: CountSymbols
    {
        let text = Printer::new().print(|paper| {
            paper.print_word(&self.sub_widgets.plural);
            SymbolsCount::new().print(settings, paper, &*input);
        });
        self.sub_widgets.show_list(text, settings, ui, input);
    }
}
