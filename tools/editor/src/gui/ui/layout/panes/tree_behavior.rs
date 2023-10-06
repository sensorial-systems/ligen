use crate::gui::ui::panes::Pane;
use crate::prelude::*;

pub struct TreeBehavior {
    simplification_options: egui_tiles::SimplificationOptions
}

impl TreeBehavior {
    pub fn new() -> Self {
        let simplification_options = egui_tiles::SimplificationOptions {
            all_panes_must_have_tabs: true,
            .. Default::default()
        };
        Self { simplification_options }
    }
}

impl egui_tiles::Behavior<Box<dyn Pane>> for TreeBehavior {
    fn pane_ui(
        &mut self,
        ui: &mut egui::Ui,
        _tile_id: egui_tiles::TileId,
        pane: &mut Box<dyn Pane>,
    ) -> egui_tiles::UiResponse {
        pane.show(ui)
    }

    fn tab_title_for_pane(&mut self, pane: &Box<dyn Pane>) -> egui::WidgetText {
        pane.title().into()
    }

    fn simplification_options(&self) -> egui_tiles::SimplificationOptions {
        self.simplification_options
    }
}