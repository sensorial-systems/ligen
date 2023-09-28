mod project;
mod module;
mod import;
mod constant;
mod path;
mod literal;
mod attributes;
mod directory;
mod visibility;
mod identifier;
mod type_;
mod function;
mod object;

use egui_tiles::UiResponse;
pub use object::*;
pub use function::*;
pub use type_::*;
pub use identifier::*;
pub use visibility::*;
pub use directory::*;
pub use path::*;
pub use constant::*;
pub use import::*;
pub use attributes::*;
pub use project::*;
pub use module::*;
pub use literal::*;
use crate::gui::ui::{MenuButton, Pane, Panes};

#[derive(Default)]
pub struct Editor {
    project: ligen_ir::Project
}

impl Editor {
    pub fn new(project: ligen_ir::Project) -> Self {
        Self { project }
    }
}

pub struct EditorMenuButton;
impl MenuButton for EditorMenuButton {
    fn menu_title(&self) -> String {
        "Project".to_string()
    }
    fn show_button(&self, ui: &mut egui::Ui, panes: &mut Panes) {
        if ui.button("Open").clicked() {
            let file = rfd::FileDialog::new()
                .add_filter("ligen-ir", &["lir"])
                .pick_file();
            if let Some(file) = file {
                if let Ok(project) = ligen_ir::Project::load(file) {
                    panes.new_pane(Box::new(Editor::new(project)));
                }
            }
            ui.close_menu();
        }
    }
}

impl Pane for Editor {
    fn title(&self) -> String {
        self.project.name.to_string()
    }

    fn show(&mut self, ui: &mut egui::Ui) -> UiResponse {
        ui.add_space(ui.spacing().menu_margin.top);
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Save").clicked() {
                    let directory = self
                        .project
                        .directory
                        .display()
                        .to_string();
                    let file = rfd::FileDialog::new()
                        .add_filter("ligen-ir", &["lir"])
                        .set_directory(directory)
                        .save_file();
                    if let Some(file) = file {
                        self
                            .project
                            .save(file)
                            .ok();
                    }
                    ui.close_menu();
                }
            });
        });
        ui.separator();
        Project::new().show(ui, &mut self.project);
        UiResponse::None
    }
}