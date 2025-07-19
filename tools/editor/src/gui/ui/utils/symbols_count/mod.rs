use ligen_idl::prelude::CountSymbols;

use crate::gui::ui::editor::settings::Settings;

use super::Paper;

#[derive(Default)]
pub struct SymbolsCount;

impl SymbolsCount {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn print(&self, settings: &Settings, paper: &mut Paper, input: impl CountSymbols) -> &Self {
        if settings.display.show_symbols_count {
            paper.print_word(format!("- Symbols: {}", input.count_symbols()));
        }
        self
    }
}
