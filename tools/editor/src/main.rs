#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod prelude;
mod gui;

ligen_gui_runtime::entrypoint!("Ligen Graphical User Interface", gui::GUI, "the_canvas_id");