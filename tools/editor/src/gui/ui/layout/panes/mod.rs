use egui::Id;
use egui_tiles::{Container, Tile};
use crate::prelude::*;

mod tree_behavior;
use tree_behavior::*;

pub trait Pane {
    fn title(&self) -> String;
    fn show(&mut self, ui: &mut egui::Ui, panes: &mut PaneManager) -> egui_tiles::UiResponse;
}

pub struct Panes {
    tree: egui_tiles::Tree<Box<dyn Pane>>
}

impl Default for Panes {
    fn default() -> Self {
        let tree = egui_tiles::Tree::empty(Id::new(12345));
        Self { tree }
    }
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
            let mut pane_manager = PaneManager::new();
            self.tree.ui(&mut TreeBehavior::new(&mut pane_manager), ui);
            for pane in pane_manager.panes {
                self.new_pane(pane)
            }
        });
    }
}

#[derive(Default)]
pub struct PaneManager {
    pub panes: Vec<Box<dyn Pane>>
}

impl PaneManager {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_pane(&mut self, pane: Box<dyn Pane>) {
        self.panes.push(pane);
    }
}