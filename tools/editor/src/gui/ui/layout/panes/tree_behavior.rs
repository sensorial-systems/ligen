use crate::gui::ui::panes::Pane;
use crate::prelude::*;

use super::PaneManager;

pub struct TreeBehavior<'panes> {
    pane_manager: &'panes mut PaneManager,
    simplification_options: egui_tiles::SimplificationOptions
}

impl<'a> TreeBehavior<'a> {
    pub fn new(pane_manager: &'a mut PaneManager) -> Self {
        let simplification_options = egui_tiles::SimplificationOptions {
            all_panes_must_have_tabs: true,
            .. Default::default()
        };
        Self { pane_manager, simplification_options }
    }
}

impl egui_tiles::Behavior<Box<dyn Pane>> for TreeBehavior<'_> {
    fn pane_ui(
        &mut self,
        ui: &mut egui::Ui,
        _tile_id: egui_tiles::TileId,
        pane: &mut Box<dyn Pane>,
    ) -> egui_tiles::UiResponse {
        pane.show(ui, self.pane_manager)
    }

    fn tab_title_for_pane(&mut self, pane: &Box<dyn Pane>) -> egui::WidgetText {
        pane.title().into()
    }

    fn simplification_options(&self) -> egui_tiles::SimplificationOptions {
        self.simplification_options
    }
}