mod library;
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
pub use library::*;
pub use module::*;
pub use literal::*;
pub use interface::*;
use ligen_idl::symbols::Symbols;
use crate::gui::ui::List;

use crate::gui::ui::panes::{Pane, PaneManager};

use super::generators::Generators;
use super::settings::Settings;
use super::widget::Widget;

#[derive(Default)]
pub struct Editor {
    library: ligen_idl::Library,
    filter: String,
    settings: Settings,
    symbols: Symbols,
    generators: Generators
}

impl Editor {
    pub fn new(library: ligen_idl::Library) -> Self {
        let filter = Default::default();
        let symbols = Symbols::new(&library);
        let display_settings = Default::default();
        let generators = Default::default();
        Self { library, symbols, filter, settings: display_settings, generators }
    }
}

impl Pane for Editor {
    fn title(&self) -> String {
        self.library.identifier.to_string()
    }

    fn show(&mut self, ui: &mut egui::Ui, _panes: &mut PaneManager) -> UiResponse {
        ui.add_space(ui.spacing().menu_margin.top);
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Save").clicked() {
                    let file = rfd::FileDialog::new()
                        .add_filter("ligen-idl", &["lir"])
                        .save_file();
                    if let Some(file) = file {
                        self
                            .library
                            .save(file)
                            .ok();
                    }
                    ui.close_menu();
                }
            });
        });
        ui.separator();
        self.settings.show(ui);
        ui.separator();
        Library::new().show(&self.settings, ui, &mut self.library);
        ui.separator();
        self.generators.show(&self.settings, ui, &mut self.library);
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Filter");
            ui.text_edit_singleline(&mut self.filter);
        });
        List::new("Symbols").show(ui, self.symbols.symbols.iter_mut().filter(|symbol| symbol.to_string().contains(self.filter.as_str())), |ui, symbol| {
            ui.label(symbol.to_string());
        });
        UiResponse::None
    }
}