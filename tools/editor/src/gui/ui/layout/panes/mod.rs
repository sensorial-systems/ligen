use egui_tiles::{Container, Tile};
use crate::prelude::*;

mod tree_behavior;
use tree_behavior::*;

pub trait Pane {
    fn title(&self) -> String;
    fn show(&mut self, ui: &mut egui::Ui) -> egui_tiles::UiResponse;
}

#[derive(Default)]
pub struct Panes {
    tree: egui_tiles::Tree<Box<dyn Pane>>
}

impl Panes {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_pane(&mut self, pane: Box<dyn Pane>) {
        let pane = self.tree.tiles.insert_pane(pane);
        if let Some(root) = &self.tree.root {
            let root = self.tree.tiles.get_mut(*root);
            if let Some(Tile::Container(Container::Tabs(tabs))) = root {
                tabs.add_child(pane);
                tabs.set_active(pane);
            }
        } else {
            let children = vec![pane];
            self.tree.root = Some(self.tree.tiles.insert_tab_tile(children));
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.tree.ui(&mut TreeBehavior::new(), ui);
        });
    }
}