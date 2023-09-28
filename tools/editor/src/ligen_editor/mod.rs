pub use crate::prelude::*;

use egui::{Style, Visuals};
use ligen_ir::Project;
pub mod ui;

/// We derive Deserialize/Serialize so we can persist ligen_editor state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct LigenEditor {
    project: Option<Project>,
    #[serde(skip)]
    tree: egui_tiles::Tree<Pane>
}

impl Default for LigenEditor {
    fn default() -> Self {
        let project = Some(Default::default());

        let mut tiles = egui_tiles::Tiles::default();
        let children = vec![
            tiles.insert_pane(Pane),
            tiles.insert_pane(Pane)
        ];
        let root = tiles.insert_tab_tile(children);
        let tree = egui_tiles::Tree::new(root, tiles);

        Self { project, tree }
    }
}

impl LigenEditor {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        cc.egui_ctx.set_style(Style {
            visuals: Visuals::light(),
            ..Default::default()
        });

        // Load previous ligen_editor state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        cc
            .storage
            .and_then(|storage| eframe::get_value(storage, eframe::APP_KEY))
            .unwrap_or_default()
    }
}

impl eframe::App for LigenEditor {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Save").clicked() {
                        if let Some(project) = self.project.as_ref() {
                            let directory = project
                                    .directory
                                    .display()
                                    .to_string();
                            let file = rfd::FileDialog::new()
                                .add_filter("ligen-ir", &["lir"])
                                .set_directory(directory)
                                .save_file();
                            if let Some(file) = file {
                                project.save(file).ok();
                            }
                        }
                        ui.close_menu();
                    }
                    if ui.button("Load").clicked() {
                        if let Some(project) = self.project.as_ref() {
                            let directory = project
                                .directory
                                .display()
                                .to_string();
                            let file = rfd::FileDialog::new()
                                .add_filter("ligen-ir", &["lir"])
                                .set_directory(directory)
                                .pick_file();
                            if let Some(file) = file {
                                if let Ok(project) = Project::load(file) {
                                    self.project = Some(project);
                                }
                            }
                            ui.close_menu();
                        }
                    }
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.tree.ui(&mut TreeBehavior::new(self.project.as_mut()), ui)
        });
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}


struct Pane;

struct TreeBehavior<'a> {
    project: Option<&'a mut Project>,
    simplification_options: egui_tiles::SimplificationOptions
}

impl<'a> TreeBehavior<'a> {
    pub fn new(project: Option<&'a mut Project>) -> Self {
        let mut simplification_options = egui_tiles::SimplificationOptions::default();
        simplification_options.all_panes_must_have_tabs = true;
        Self { project, simplification_options }
    }
}

impl egui_tiles::Behavior<Pane> for TreeBehavior<'_> {
    fn pane_ui(
        &mut self,
        ui: &mut egui::Ui,
        _tile_id: egui_tiles::TileId,
        _pane: &mut Pane,
    ) -> egui_tiles::UiResponse {
        if let Some(project) = &mut self.project {
            ui::Project::new().show(ui, project);
        }
        egui_tiles::UiResponse::None
    }

    fn tab_title_for_pane(&mut self, _pane: &Pane) -> egui::WidgetText {
        self.project.as_ref().map(|project| project.name.to_string()).unwrap_or_default().into()
    }

    fn simplification_options(&self) -> egui_tiles::SimplificationOptions {
        self.simplification_options
    }
}