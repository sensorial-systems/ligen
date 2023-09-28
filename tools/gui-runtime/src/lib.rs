#![warn(clippy::all, rust_2018_idioms)]

#[macro_export]
macro_rules! entrypoint {
    ($title:expr, $eframe_app:ty, $canvas_id:expr) => {
        // When compiling natively:
        #[cfg(not(target_arch = "wasm32"))]
        fn main() -> $crate::eframe::Result<()> {
            $crate::env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

            let native_options = $crate::eframe::NativeOptions::default();
            $crate::eframe::run_native(
                $title,
                native_options,
                Box::new(|cc| Box::new(<$eframe_app>::new(cc))),
            )
        }

        // When compiling to web using trunk:
        #[cfg(target_arch = "wasm32")]
        fn main() {
            // Redirect `log` message to `console.log` and friends:
            $crate::eframe::WebLogger::init(log::LevelFilter::Debug).ok();

            let web_options = $crate::eframe::WebOptions::default();

            $crate::wasm_bindgen_futures::spawn_local(async {
                $crate::eframe::WebRunner::new()
                    .start(
                        $canvas_id,
                        web_options,
                        Box::new(|cc| Box::new(<$eframe_app>::new(cc))),
                    )
                    .await
                    .expect("failed to start eframe");
            });
        }
    };
}

pub use egui;
#[cfg(not(target_arch = "wasm32"))]
mod reexports {
    pub use env_logger;
    pub use rfd;
}

#[cfg(target_arch = "wasm32")]
mod reexports {
    pub use wasm_bindgen_futures;
}

pub use reexports::*;
pub use eframe;
