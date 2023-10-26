mod project;
mod module;
mod import;
mod path;
mod literal;
mod attributes;
mod directory;
mod visibility;
mod identifier;
mod type_;
mod function;
mod object;
mod interface;
mod menu_button;

use egui_tiles::UiResponse;
pub use function::*;
pub use type_::*;
pub use identifier::*;
pub use visibility::*;
pub use directory::*;
pub use path::*;
pub use object::*;
pub use import::*;
pub use attributes::*;
pub use project::*;
pub use module::*;
pub use literal::*;
pub use menu_button::*;
pub use interface::*;
use ligen_ir::symbols::Symbols;
use crate::gui::ui::List;

use crate::gui::ui::panes::Pane;

#[derive(Default)]
pub struct Editor {
    project: ligen_ir::Project,
    filter: String,
    symbols: Symbols
}

impl Editor {
    pub fn new(project: ligen_ir::Project) -> Self {
        let filter = Default::default();
        let symbols = Symbols::new(&project);
        Self { project, symbols, filter }
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
                    let file = rfd::FileDialog::new()
                        .add_filter("ligen-ir", &["lir"])
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
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Filter");
            ui.text_edit_singleline(&mut self.filter);
        });
        List::new("Symbols").show(ui, &mut self.symbols.symbols.iter_mut().filter(|symbol| symbol.to_string().contains(self.filter.as_str())), |ui, symbol| {
            ui.label(symbol.to_string());
        });
        UiResponse::None
    }
}