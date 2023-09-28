#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod prelude;
mod ligen_editor;

ligen_gui_runtime::entrypoint!("Ligen Editor", ligen_editor::LigenEditor, "the_canvas_id");